mod parsers;
mod tokenizer;
mod filters;

extern crate core;

use std::cmp::{max, min};
use clap::{Arg, Command};
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use crate::filters::{AndFilters, AnyFieldsContains, CiteKeyContains, EntryTypeContains, FieldContains, Filter, NegateFilter};
use crate::parsers::BibTexParser;
use crate::tokenizer::Tokenizer;

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
        .get_matches();

    let mut entries = Vec::new();
    for filename in app.values_of("filenames").unwrap() {
        let bib_file = File::open(filename)?;
        let mut reader = BufReader::new(bib_file);
        let mut bib = String::new();
        reader.read_to_string(&mut bib)?;
        let mut tokenizer = Tokenizer::new(bib.chars());
        let mut parser = BibTexParser::new(&mut tokenizer);

        let mut file_entries = parser.entries();
        entries.append(&mut file_entries);
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
    sorted_entries.sort_by_key(|it| it.fields.get("year").map(|it| it.parse::<i32>().unwrap_or(0)));
    if app.is_present("decreasing") {
        sorted_entries.reverse();
    }

    if app.is_present("tabular") {
        let mut col_width = vec![10, 10, 20, 10, 4];
        for entry in &selected_entries {
            let authors = entry.fields.get("author").cloned().unwrap_or(String::new());
            for author in authors.split(" and ") {
                col_width[3] = max(col_width[3], author.len());
            }
            col_width[0] = max(col_width[0], entry.entry_type.len());
            col_width[1] = max(col_width[1], entry.cite_key.len());
            col_width[2] = max(col_width[2], entry.fields.get("title").unwrap_or(&String::new()).len());
            col_width[4] = max(col_width[4], entry.fields.get("year").unwrap_or(&String::new()).len());
        }

        col_width[2] = min(col_width[2], 60);

        println!("+-{:->col0$}-+-{:->col1$}-+-{:->col2$}-+-{:->col3$}-+-{:->col4$}-+",
                 "",
                 "",
                 "",
                 "",
                 "",
                 col0 = col_width[0],
                 col1 = col_width[1],
                 col2 = col_width[2],
                 col3 = col_width[3],
                 col4 = col_width[4],
        );

        println!("+ {:col0$} + {:col1$} + {:col2$} + {:col3$} + {:col4$} +",
                 "Type",
                 "Cite key",
                 "Title",
                 "Author(s)",
                 "Year",
                 col0 = col_width[0],
                 col1 = col_width[1],
                 col2 = col_width[2],
                 col3 = col_width[3],
                 col4 = col_width[4],
        );

        println!("+-{:->col0$}-+-{:->col1$}-+-{:->col2$}-+-{:->col3$}-+-{:->col4$}-+",
                   "",
                   "",
                   "",
                   "",
                   "",
                   col0 = col_width[0],
                   col1 = col_width[1],
                   col2 = col_width[2],
                   col3 = col_width[3],
                   col4 = col_width[4],
        );

        let empty_string = String::new();
        for entry in &sorted_entries {
            let author= entry.fields.get("author").cloned().unwrap_or(String::new());
            let authors = author.split(" and ").collect::<Vec<_>>();
            let title_lines = entry.fields.get("title").map(|it| multiline(it, 60)).unwrap_or(Vec::new());

            let nb_lines = max(authors.len(), title_lines.len());
            for i in 0..nb_lines {
                if i == 0 {
                    println!("| {:col0$} | {:col1$} | {:col2$} | {:col3$} | {:col4$} |",
                             entry.entry_type,
                             entry.cite_key,
                             if i < title_lines.len() { &title_lines[i] } else { &empty_string },
                             if i < authors.len() { authors[i] } else { "" },
                             entry.fields.get("year").unwrap_or(&String::new()),
                             col0 = col_width[0],
                             col1 = col_width[1],
                             col2 = col_width[2],
                             col3 = col_width[3],
                             col4 = col_width[4],
                    );
                } else {
                    println!("| {:col0$} | {:col1$} | {:col2$} | {:col3$} | {:col4$} |",
                             "",
                             "",
                             if i < title_lines.len() { &title_lines[i] } else { &empty_string },
                             if i < authors.len() { authors[i] } else { ""},
                             "",
                             col0 = col_width[0],
                             col1 = col_width[1],
                             col2 = col_width[2],
                             col3 = col_width[3],
                             col4 = col_width[4],
                    );
                }
            }
            println!("+-{:->col0$}-+-{:->col1$}-+-{:->col2$}-+-{:->col3$}-+-{:->col4$}-+",
                     "",
                     "",
                     "",
                     "",
                     "",
                     col0 = col_width[0],
                     col1 = col_width[1],
                     col2 = col_width[2],
                     col3 = col_width[3],
                     col4 = col_width[4],
            );
        }
    } else {
        println!("{:#?}", selected_entries);
    }

    println!("{}", selected_entries.len());
    Ok(())
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BibTexEntry {
    pub entry_type: String,
    pub cite_key: String,
    pub fields: BTreeMap<String, String>,
}

pub fn parser_query(query: &str) -> AndFilters<BibTexEntry> {
    println!("{:?}", query);

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

fn multiline(line: &str, max_len: usize) -> Vec<String> {
    let mut lines = Vec::new();

    let mut buffer = String::new();
    for part in line.split(|it: char| it.is_whitespace()) {
        if !buffer.is_empty() {
            buffer.push(' ')
        }
        if buffer.len() + part.len() >= max_len {
            lines.push(buffer);
            buffer = String::new();
        }
        buffer.push_str(part)
    }
    if !buffer.is_empty() {
        lines.push(buffer);
    }

    lines
}