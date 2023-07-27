use crate::tokenizer::{ANY, CLOSE_BRACKET, OPEN_BRACKET, Tokenizer};

pub struct BibTexCommentEntryParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> BibTexCommentEntryParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> BibTexCommentEntryParser<'t, 'c> {
        BibTexCommentEntryParser { tokenizer }
    }

    // 'COMMENT{' CONTENT '}'
    pub fn skip_comment(&mut self) -> () {
        self.tokenizer.skip(&|it| it == 'C');
        self.tokenizer.skip(&|it| it == 'O');
        self.tokenizer.skip(&|it| it == 'M');
        self.tokenizer.skip(&|it| it == 'M');
        self.tokenizer.skip(&|it| it == 'E');
        self.tokenizer.skip(&|it| it == 'N');
        self.tokenizer.skip(&|it| it == 'T');
        self.tokenizer.skip(&OPEN_BRACKET);

        let mut depth = 1;
        while depth > 0 {
            let lookahead = self.tokenizer.consume(&ANY);
            if OPEN_BRACKET(lookahead) {
                depth += 1;
            } else if CLOSE_BRACKET(lookahead) {
                depth -= 1;
            }
        }
    }
}