use crate::{Content, Tokenizer};
use crate::Content::Inlined;
use crate::tokenizer::DIGIT;

pub struct IntegerParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> IntegerParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> IntegerParser<'t, 'c> {
        IntegerParser { tokenizer }
    }

    // [0-9]*
    pub fn number(&mut self) -> Content {
        Inlined(self.tokenizer.consume_while(&DIGIT))
    }
}