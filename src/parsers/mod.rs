use crate::BibTexEntry;
use crate::parsers::bibtex_entry::BibTexEntryParser;
use crate::tokenizer::Tokenizer;

pub mod bibtex_entry;
pub mod entry_type;
pub mod cite_key;
pub mod field_parser;
pub mod filed_name_parser;
pub mod integer_parser;
pub mod content_parser;
pub mod word_parser;
pub mod quoted_content_parser;
pub mod braced_content_parser;
pub mod content;

pub struct BibTexParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>
}

impl <'t, 'c: 't> BibTexParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> BibTexParser<'t, 'c> {
        BibTexParser { tokenizer }
    }

    pub fn entries(&mut self) -> Vec<BibTexEntry> {
        let mut entries = Vec::new();
        while !self.tokenizer.eof {
            entries.push(BibTexEntryParser::new(self.tokenizer).entry());
        }
        entries
    }
}

