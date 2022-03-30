use crate::parsers::braced_content_parser::BracedContentParser;
use crate::parsers::integer_parser::IntegerParser;
use crate::parsers::quoted_content_parser::QuotedContentParser;
use crate::parsers::word_parser::WordParser;
use crate::{Content, Tokenizer};
use crate::tokenizer::{ALPHA, DIGIT, DOUBLE_QUOTE, OPEN_BRACKET};

pub struct ContentParser<'t, 'c> {
    pub tokenizer: &'t mut Tokenizer<'c>
}
impl <'t, 'c: 't> ContentParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> ContentParser<'t, 'c> {
        ContentParser { tokenizer }
    }

    // DOUBLE_QUOTE_CONTENT | BRACED_CONTENT | word | integer
    pub fn content(&mut self) -> Content {
        if DOUBLE_QUOTE(self.tokenizer.lookahead) {
            QuotedContentParser::new(self.tokenizer).quoted_content()
        } else if OPEN_BRACKET(self.tokenizer.lookahead) {
            BracedContentParser::new(self.tokenizer).braced_content()
        } else if ALPHA(self.tokenizer.lookahead) {
            WordParser::new(self.tokenizer).word()
        } else if DIGIT(self.tokenizer.lookahead) {
            IntegerParser::new(self.tokenizer).number()
        } else {
            panic!("")
        }
    }
}