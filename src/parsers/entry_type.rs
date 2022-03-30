use crate::Tokenizer;

pub struct EntryTypeParser<'t, 'c> {
    pub tokenizer: &'t mut Tokenizer<'c>
}

type EntryType = String;

impl <'t, 'c: 't> EntryTypeParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> EntryTypeParser<'t, 'c> {
        EntryTypeParser { tokenizer }
    }

    // [a-zA-Z0-9-_+/:]
    pub fn entry_type(&mut self) -> EntryType {
        self.tokenizer.consume_while(&IS_ENTRY_VALID_CHARACTER)
    }
}

#[allow(non_snake_case)]
fn IS_ENTRY_VALID_CHARACTER(it: char) -> bool {
    it.is_alphanumeric() || ['-', '_', '+', '/', ':'].contains(&it)
}