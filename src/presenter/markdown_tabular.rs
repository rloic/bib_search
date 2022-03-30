use std::cmp::{max, min};
use crate::BibTexEntry;

pub struct Presenter {}

impl crate::presenter::Presenter<Vec<&&BibTexEntry>> for Presenter {
    fn present(&self, data: &Vec<&&BibTexEntry>) {
        let mut col_width = vec![10, 10, 20, 10, 4];
        for entry in data {
            let authors = entry.fields.get("author").map(|it| it.to_string()).unwrap_or(String::new());
            for author in authors.split(" and ") {
                col_width[3] = max(col_width[3], author.len());
            }
            col_width[0] = max(col_width[0], entry.entry_type.len());
            col_width[1] = max(col_width[1], entry.cite_key.len());
            col_width[2] = max(col_width[2], entry.fields.get("title").map(|it| it.to_string()).unwrap_or(String::new()).len());
            col_width[4] = max(col_width[4], entry.fields.get("year").map(|it| it.to_string()).unwrap_or(String::new()).len());
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
        for entry in data {
            let author= entry.fields.get("author").map(|it| it.to_string()).unwrap_or(String::new());
            let authors = author.split(" and ").collect::<Vec<_>>();
            let title_lines = entry.fields.get("title").map(|it| multiline(&it.to_string(), 60)).unwrap_or(Vec::new());

            let nb_lines = max(authors.len(), title_lines.len());
            for i in 0..nb_lines {
                if i == 0 {
                    println!("| {:col0$} | {:col1$} | {:col2$} | {:col3$} | {:col4$} |",
                             entry.entry_type,
                             entry.cite_key,
                             if i < title_lines.len() { &title_lines[i] } else { &empty_string },
                             if i < authors.len() { authors[i] } else { "" },
                             entry.fields.get("year").map(|it| it.to_string()).unwrap_or(String::new()),
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
    }
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