use crate::Tokenizer;

pub struct FieldNameParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> FieldNameParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> FieldNameParser<'t, 'c> {
        FieldNameParser { tokenizer }
    }

    // [a-zA-Z0-9]*
    pub fn field_name(&mut self) -> String {
        self.tokenizer.consume_while(&IS_VALID_FIELD_NAME_CHARACTER).to_lowercase()
    }
}

#[allow(non_snake_case)]
fn IS_VALID_FIELD_NAME_CHARACTER(it: char) -> bool {
    it.is_alphanumeric() || ['-', '_'].contains(&it)
}