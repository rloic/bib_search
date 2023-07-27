use crate::{Tokenizer};
use crate::Content::Concatenated;
use crate::parsers::content_parser::ContentParser;
use crate::parsers::field::Field;
use crate::tokenizer::{ASSIGN, CONCAT};

type CiteKey = String;
pub enum Either<L, R> {
    Left(L), Right(R)
}

pub struct CiteKeyParser<'t, 'c> {
    pub tokenizer: &'t mut Tokenizer<'c>
}
impl <'t, 'c: 't> CiteKeyParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> CiteKeyParser<'t, 'c> {
        CiteKeyParser { tokenizer }
    }

    // [a-zA-Z0-9-_+/:.]*
    pub fn cite_key_or_field(&mut self) -> Either<CiteKey, Field> {
        let cite_key = self.tokenizer.consume_while(&IS_CITE_KEY_CHARACTER);
        if ASSIGN(self.tokenizer.lookahead) {
            self.tokenizer.skip(&ASSIGN);
            let mut contents = Vec::new();
            contents.push(ContentParser::new(self.tokenizer).content());
            while CONCAT(self.tokenizer.lookahead) {
                self.tokenizer.skip(&CONCAT);
                contents.push(ContentParser::new(self.tokenizer).content());
            }
            Either::Right(Field(cite_key, Concatenated(contents)))
        } else {
            Either::Left(cite_key)
        }
    }

}

#[allow(non_snake_case)]
fn IS_CITE_KEY_CHARACTER(c: char) -> bool {
    c.is_alphanumeric() || ['-', '_', '+', '/', ':', '.'].contains(&c)
}