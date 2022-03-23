use crate::Tokenizer;
use crate::tokenizer::{ANY, DOUBLE_QUOTE, ESCAPED, WS};

pub struct QuotedContentParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> QuotedContentParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> QuotedContentParser<'t, 'c> {
        QuotedContentParser { tokenizer }
    }

    // '"' [^"] '"'
    pub fn quoted_content(&mut self) -> String {
        self.tokenizer.skip_ws = false;
        self.tokenizer.skip_comment = false;
        self.tokenizer.skip(&DOUBLE_QUOTE);

        let mut text = String::new();

        while !DOUBLE_QUOTE(self.tokenizer.lookahead) {
            if WS(self.tokenizer.lookahead) {
                self.tokenizer.skip_while(&WS);
                text.push(' ')
            } else {
                if ESCAPED(self.tokenizer.lookahead) {
                    text.push(self.tokenizer.consume(&ESCAPED));
                }
                text.push(self.tokenizer.consume(&ANY));
            }
        }

        self.tokenizer.skip_comment = true;
        self.tokenizer.skip_ws = true;
        self.tokenizer.skip(&DOUBLE_QUOTE);
        text
    }
}