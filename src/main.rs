mod parsers;
mod tokenizer;
mod filters;
mod presenter;

extern crate core;

use clap::{Arg, Command};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use crate::filters::{AndFilters, AnyFieldsContains, CiteKeyContains, EntryTypeContains, FieldContains, Filter, NegateFilter};
use crate::parsers::BibTexParser;
use crate::parsers::content::Content;
use crate::presenter::Presenter;
use crate::tokenizer::{Tokenizer};

fn main() -> std::io::Result<()> {
    let app = Command::new("bib_search")
        .arg(Arg::new("filenames")
            .required(true)
            .takes_value(true)
            .min_values(1))
        .arg(Arg::new("queries")
            .long("query").short('q')
            .required(true)
            .takes_value(true)
            .min_values(1))
        .arg(Arg::new("tabular")
            .long("tabular").short('t'))
        .arg(Arg::new("decreasing")
            .long("decreasing").short('d'))
        .arg(Arg::new("count")
            .long("count").short('c'))
        .get_matches();

    let mut entries = Vec::new();
    let mut files = Vec::new();
    for filename in app.values_of("filenames").unwrap() {
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

    let mut selected_entries = HashSet::new();
    for query in app.values_of("queries").unwrap() {
        let filter = parser_query(query);
        for entry in &entries {
            if filter.accept(&entry) {
                selected_entries.insert(entry);
            }
        }
    }

    let mut sorted_entries = selected_entries.iter().collect::<Vec<_>>();
    sorted_entries.sort_by_key(|it| it.fields.get("year").map(|it| it.to_string().parse::<i32>().unwrap_or(0)));
    if app.is_present("decreasing") {
        sorted_entries.reverse();
    }

    if app.is_present("tabular") {
        let presenter = presenter::markdown_tabular::Presenter{};
        presenter.present(&sorted_entries);
    } else {
        let presenter = presenter::bibtex::Presenter{};
        presenter.present(&sorted_entries);
    }

    if app.is_present("count") {
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
            "t" => "title",
            "e" => "editor",
            "p" => "publisher",
            "a" => "author",
            "y" => "year",
            _ => field_name
        };

        let filter: Box<dyn Filter<BibTexEntry>> = match field_name {
            "*" => Box::new(AnyFieldsContains::new(value)),
            "entry_type" => Box::new(EntryTypeContains::new(value)),
            "c" | "cite_key" => Box::new(CiteKeyContains::new(value)),
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