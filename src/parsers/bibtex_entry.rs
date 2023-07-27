use std::collections::{BTreeMap};
use crate::parsers::cite_key::{CiteKeyParser, Either};
use crate::parsers::entry_type::EntryTypeParser;
use crate::parsers::field_parser::FieldParser;
use crate::{BibTexEntry, Tokenizer};
use crate::parsers::field::Field;
use crate::tokenizer::{AT_SIGN, CLOSE_BRACKET, COMMA, OPEN_BRACKET};

pub struct BibTexEntryParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>
}

impl <'t, 'c: 't> BibTexEntryParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> BibTexEntryParser<'t, 'c> {
        BibTexEntryParser { tokenizer }
    }

    // '@' ENTRY_TYPE '{' CITE_KEY ',' (field ','?)* '}'
    pub fn entry(&mut self) -> BibTexEntry {
        self.tokenizer.skip(&AT_SIGN);
        let entry_type = EntryTypeParser::new(self.tokenizer).entry_type();
        self.tokenizer.skip(&OPEN_BRACKET);
        let mut fields = BTreeMap::new();
        let mut cite_key = String::new();
        match CiteKeyParser::new(self.tokenizer).cite_key_or_field() {
            Either::Left(inner) => { cite_key = inner; },
            Either::Right(Field(key, value)) => { fields.insert(key, value); }
        };
        self.tokenizer.skip_optional(&COMMA);

        while !CLOSE_BRACKET(self.tokenizer.lookahead) {
            let Field(key, value) = FieldParser::new(self.tokenizer).field();
            self.tokenizer.skip_optional(&COMMA);
            fields.insert(key, value);
        }

        self.tokenizer.skip(&CLOSE_BRACKET);
        BibTexEntry { entry_type, cite_key, fields  }
    }

}