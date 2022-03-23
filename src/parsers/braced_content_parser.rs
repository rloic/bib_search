use crate::Tokenizer;
use crate::tokenizer::{ANY, CLOSE_BRACKET, ESCAPED, OPEN_BRACKET, WS};

pub struct BracedContentParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> BracedContentParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> BracedContentParser<'t, 'c> {
        BracedContentParser { tokenizer }
    }

    // '{' text '}'
    pub fn braced_content(&mut self) -> String {
        self.tokenizer.skip_ws = false;
        self.tokenizer.skip_comment = false;
        self.tokenizer.skip(&OPEN_BRACKET);

        let mut text = String::new();
        let mut stack = Vec::new();
        stack.push('{');

        while !stack.is_empty() {
            if WS(self.tokenizer.lookahead) {
                self.tokenizer.skip_while(&WS);
                text.push(' ');
                continue;
            }
            if ESCAPED(self.tokenizer.lookahead) {
                text.push(self.tokenizer.consume(&ESCAPED));
                text.push(self.tokenizer.consume(&ANY));
                continue;
            }
            if OPEN_BRACKET(self.tokenizer.lookahead) {
                stack.push('{');
            } else if CLOSE_BRACKET(self.tokenizer.lookahead) && stack.last().map(|it| OPEN_BRACKET(*it)).unwrap_or(false) {
                stack.pop();
            }
            if !stack.is_empty() {
                text.push(self.tokenizer.consume(&ANY));
            }
        }

        self.tokenizer.skip_comment = true;
        self.tokenizer.skip_ws = true;
        self.tokenizer.skip(&CLOSE_BRACKET);
        text
    }
}