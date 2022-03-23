use crate::Tokenizer;

type CiteKey = String;

pub struct CiteKeyParser<'t, 'c> {
    pub tokenizer: &'t mut Tokenizer<'c>
}
impl <'t, 'c: 't> CiteKeyParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> CiteKeyParser<'t, 'c> {
        CiteKeyParser { tokenizer }
    }

    // [a-zA-Z0-9-_+/:.]*
    pub fn cite_key(&mut self) -> CiteKey {
        self.tokenizer.consume_while(&IS_CITE_KEY_CHARACTER)
    }

}

#[allow(non_snake_case)]
fn IS_CITE_KEY_CHARACTER(c: char) -> bool {
    c.is_alphanumeric() || ['-', '_', '+', '/', ':', '.'].contains(&c)
}