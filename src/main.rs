mod parsers;
mod tokenizer;
mod filters;
mod presenter;

extern crate core;

use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use clap::{Arg, Command};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use regex::Regex;
use sqlparser::ast::{BinaryOperator, Expr, Query, UnaryOperator, Value};
use sqlparser::ast::BinaryOperator::{And, Eq, Gt, GtEq, Lt, LtEq, NotEq, Or, Xor};
use sqlparser::dialect::GenericDialect;
use crate::filters::{AndFilters, AnyFieldsContains, CiteKeyContains, EntryTypeContains, FieldContains, Filter, NegateFilter, OrFilters};
use crate::parsers::BibTexParser;
use crate::parsers::content::Content;
use crate::presenter::Presenter;
use crate::tokenizer::{Tokenizer};

use clap::Parser;


#[derive(Parser)]
struct Args {
    filenames: Vec<PathBuf>,
    #[arg(short, long)]
    query: Option<String>,
    #[arg(short, long)]
    tabular: bool,
    #[arg(short, long)]
    count: bool,
}

fn main() -> std::io::Result<()> {
    let app: Args = Args::parse();

    let mut entries = Vec::new();
    let mut files = Vec::new();

    if !app.filenames.is_empty() {
        for filename in app.filenames {
            let bib_file = File::open(filename)?;
            {
                let mut reader = BufReader::new(&bib_file);
                let mut bib = String::new();
                reader.read_to_string(&mut bib)?;
                let mut tokenizer = Tokenizer::new(bib.chars());
                let mut parser = BibTexParser::new(&mut tokenizer);

                let mut file_entries = parser.entries();
                entries.append(&mut file_entries);
            }
            files.push(bib_file);
        }
    } else {
        let mut bib = String::new();
        std::io::stdin().read_to_string(&mut bib)?;
        let mut tokenizer = Tokenizer::new(bib.chars());
        let mut parser = BibTexParser::new(&mut tokenizer);

        let mut file_entries = parser.entries();
        entries.append(&mut file_entries);
    }

    let mut selected_entries = HashSet::new();
    if let Some(query) = app.query {
        let mut ast = sqlparser::parser::Parser::new(&GenericDialect)
            .try_with_sql(&query)
            .unwrap();
        let where_clause = ast_to_predicate(ast.parse_expr().unwrap());

        for entry in &entries {
            if where_clause.accept(entry) {
                selected_entries.insert(entry);
            }
        }
    } else {
        for entry in &entries {
            selected_entries.insert(entry);
        }
    }

    let mut sorted_entries = selected_entries.iter().collect::<Vec<_>>();
    sorted_entries.sort_by_key(|it| it.fields.get("year").map(|it| it.to_string().parse::<i32>().unwrap_or(0)));
    /*if app.is_present("decreasing") {
        sorted_entries.reverse();
    }*/

    if app.tabular {
        let presenter = presenter::markdown_tabular::Presenter {};
        presenter.present(&sorted_entries);
    } else {
        let presenter = presenter::bibtex::Presenter {};
        presenter.present(&sorted_entries);
    }

    if app.count {
        println!("{}", selected_entries.len());
    }
    Ok(())
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BibTexEntry {
    pub entry_type: String,
    pub cite_key: String,
    pub fields: BTreeMap<String, Content>,
}

pub fn ast_to_predicate(ast: Expr) -> Box<dyn Filter<BibTexEntry>> {
    match ast {
        Expr::Nested(ast) => ast_to_predicate(*ast),
        Expr::BinaryOp { left, op, right } => {
            match op {
                Eq => Box::new(EqFilter { lhs: ast_to_selector(*left), rhs: ast_to_selector(*right) }),
                NotEq => Box::new(NeqFilter { lhs: ast_to_selector(*left), rhs: ast_to_selector(*right) }),
                And => { Box::new(AndFilters::new(vec![ast_to_predicate(*left), ast_to_predicate(*right)])) }
                Or => { Box::new(OrFilters::new(vec![ast_to_predicate(*left), ast_to_predicate(*right)])) }
                Gt => Box::new(CmpFilter { lhs: ast_to_selector(*left), order: Greater, rhs: ast_to_selector(*right) }),
                Lt => Box::new(CmpFilter { lhs: ast_to_selector(*left), order: Less, rhs: ast_to_selector(*right) }),
                GtEq => Box::new(CmpInvFilter { lhs: ast_to_selector(*left), order: Less, rhs: ast_to_selector(*right) }),
                LtEq => Box::new(CmpInvFilter { lhs: ast_to_selector(*left), order: Greater, rhs: ast_to_selector(*right) }),
                _ => unimplemented!("unsupported operator {:?}", op)
            }
        }
        Expr::Like { expr, negated, pattern, .. } => {
            match pattern.as_ref() {
                Expr::Value(value) => {
                    match value {
                        Value::SingleQuotedString(inner) => {
                            let re: String = String::from("^") + &inner.replace("%", ".*").replace("_", ".") + "$";
                            Box::new(LikeFilter { lhs: ast_to_selector(*expr), expr: Regex::new(&re).unwrap(), negated })
                        }
                        _ => unimplemented!()
                    }
                }
                _ => unimplemented!()
            }
        }
        Expr::ILike { expr, negated, pattern, .. } => {
            match pattern.as_ref() {
                Expr::Value(value) => {
                    match value {
                        Value::SingleQuotedString(inner) => {
                            let re: String = String::from("^(?i)") + &inner.replace("%", ".*").replace("_", ".") + "$";
                            Box::new(LikeFilter { lhs: ast_to_selector(*expr), expr: Regex::new(&re).unwrap(), negated })
                        }
                        _ => unimplemented!()
                    }
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!("unsupported expression {:?}", ast)
    }
}

struct CmpFilter {
    lhs: Box<dyn Selector<BibTexEntry>>,
    order: Ordering,
    rhs: Box<dyn Selector<BibTexEntry>>,
}

impl Filter<BibTexEntry> for CmpFilter {
    fn accept(&self, element: &BibTexEntry) -> bool {
        let (lhs, rhs) = (self.lhs.select(element), self.rhs.select(element));
        match (lhs, rhs) {
            (Some(l_value), Some(r_value)) => natord::compare(&l_value, &r_value) == self.order,
            _ => false
        }
    }
}

struct CmpInvFilter {
    lhs: Box<dyn Selector<BibTexEntry>>,
    order: Ordering,
    rhs: Box<dyn Selector<BibTexEntry>>,
}

impl Filter<BibTexEntry> for CmpInvFilter {
    fn accept(&self, element: &BibTexEntry) -> bool {
        let (lhs, rhs) = (self.lhs.select(element), self.rhs.select(element));
        match (lhs, rhs) {
            (Some(l_value), Some(r_value)) => natord::compare(&l_value, &r_value) != self.order,
            _ => false
        }
    }
}

struct LikeFilter {
    lhs: Box<dyn Selector<BibTexEntry>>,
    expr: Regex,
    negated: bool,
}

impl Filter<BibTexEntry> for LikeFilter {
    fn accept(&self, element: &BibTexEntry) -> bool {
        if let Some(value) = self.lhs.select(element) {
            self.expr.is_match(&value) != self.negated
        } else {
            self.negated
        }
    }
}

struct EqFilter {
    lhs: Box<dyn Selector<BibTexEntry>>,
    rhs: Box<dyn Selector<BibTexEntry>>,
}

impl Filter<BibTexEntry> for EqFilter {
    fn accept(&self, element: &BibTexEntry) -> bool {
        self.lhs.select(element) == self.rhs.select(element)
    }
}

struct NeqFilter {
    lhs: Box<dyn Selector<BibTexEntry>>,
    rhs: Box<dyn Selector<BibTexEntry>>,
}

impl Filter<BibTexEntry> for NeqFilter {
    fn accept(&self, element: &BibTexEntry) -> bool {
        self.lhs.select(element) != self.rhs.select(element)
    }
}

pub trait Selector<T> {
    fn select(&self, element: &T) -> Option<String>;
}

struct FieldSelector {
    field_name: String,
}

impl Selector<BibTexEntry> for Option<String> {
    fn select(&self, _: &BibTexEntry) -> Option<String> {
        self.clone()
    }
}

impl Selector<BibTexEntry> for FieldSelector {
    fn select(&self, element: &BibTexEntry) -> Option<String> {
        match self.field_name.as_str() {
            "entry_type" => Some(element.entry_type.clone()),
            "cite_key" => Some(element.cite_key.clone()),
            field => element.fields.get(field)
                .map(|it| it.to_string())
        }
    }
}

pub fn ast_to_selector(expr: Expr) -> Box<dyn Selector<BibTexEntry>> {
    match expr {
        Expr::Identifier(ident) => Box::new(FieldSelector { field_name: ident.value }),
        Expr::Value(value) => match value {
            sqlparser::ast::Value::Number(inner, _) => Box::new(Some(inner)),
            sqlparser::ast::Value::Boolean(b) => Box::new(Some(b.to_string())),
            sqlparser::ast::Value::SingleQuotedString(inner) => Box::new(Some(inner)),
            _ => unimplemented!("unsupported value {:?}", value)
        },
        _ => unimplemented!("unsupported expression {:?}", expr)
    }
}

pub fn parser_query(query: &str) -> AndFilters<BibTexEntry> {
    let mut filters: Vec<Box<dyn Filter<BibTexEntry>>> = Vec::new();
    let lower_case = query.to_lowercase();
    let suq_queries = lower_case.split('&');

    for sub_query in suq_queries {
        let (mut field_name, value) = sub_query.split_once(':')
            .expect("Query must be of format: 'key:value'");

        let mut accepts = true;
        if field_name.starts_with('!') {
            accepts = false;
            field_name = &field_name[1..];
        }

        let field_name = match field_name {
            "@" => "entry_type",
            "t" => "title",
            "e" => "editor",
            "p" => "publisher",
            "a" => "author",
            "y" => "year",
            "c" => "cite_key",
            _ => field_name
        };

        let filter: Box<dyn Filter<BibTexEntry>> = match field_name {
            "*" => Box::new(AnyFieldsContains::new(value)),
            "entry_type" => Box::new(EntryTypeContains::new(value)),
            "cite_key" => Box::new(CiteKeyContains::new(value)),
            _ => Box::new(FieldContains::new(field_name, value))
        };

        if accepts {
            filters.push(filter);
        } else {
            filters.push(Box::new(NegateFilter::new(filter)));
        }
    }

    AndFilters::new(filters)
}