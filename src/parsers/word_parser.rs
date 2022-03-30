use crate::Content::Inlined;
use crate::{Content, Tokenizer};
use crate::tokenizer::{ALPHA};

pub struct WordParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> WordParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> WordParser<'t, 'c> {
        WordParser { tokenizer }
    }

    // [a-zA-Z][a-zA-Z0-9-_:/+]*
    pub fn word(&mut self) -> Content {
        let mut word = String::new();
        word.push(self.tokenizer.consume(&IS_FIRST_WORD_CHARACER));
        word.push_str(&self.tokenizer.consume_while(&IS_NTH_WORD_CHARACTER));
        Inlined(word)
    }
}

#[allow(non_snake_case)]
fn IS_FIRST_WORD_CHARACER(it: char) -> bool {
    ALPHA(it)
}

#[allow(non_snake_case)]
fn IS_NTH_WORD_CHARACTER(it: char) -> bool {
    it.is_alphanumeric() || ['-', '_', ':', '/', '+'].contains(&it)
}