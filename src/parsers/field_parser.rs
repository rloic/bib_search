use crate::parsers::content_parser::ContentParser;
use crate::parsers::filed_name_parser::FieldNameParser;
use crate::Tokenizer;
use crate::tokenizer::{ASSIGN, CONCAT};

pub struct FieldParser<'t, 'c> {
    tokenizer: &'t mut Tokenizer<'c>,
}

impl<'t, 'c: 't> FieldParser<'t, 'c> {
    pub fn new(tokenizer: &'t mut Tokenizer<'c>) -> FieldParser<'t, 'c> {
        FieldParser { tokenizer }
    }

    // title '=' content ('#' content)*
    pub fn field(&mut self) -> (String, String) {
        let title = FieldNameParser::new(self.tokenizer).field_name();
        self.tokenizer.skip(&ASSIGN);
        let mut content = ContentParser::new(self.tokenizer).content();
        while CONCAT(self.tokenizer.lookahead) {
            self.tokenizer.skip(&CONCAT);
            content.push(' ');
            content.push_str(&ContentParser::new(self.tokenizer).content());
        }
        (title, content)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::field_parser::FieldParser;
    use crate::Tokenizer;

    #[test]
    fn parse_field_with_int_value() {
        let bib_tex_content = "field-name = 10";
        let mut tokenizer = Tokenizer::new(bib_tex_content.chars());
        let mut parser = FieldParser::new(&mut tokenizer);
        let (field_name, content) = parser.field();
        assert!(tokenizer.eof);
        assert_eq!("field-name", field_name.as_str());
        assert_eq!("10", content.as_str());
    }

    #[test]
    fn field_with_quoted_content() {
        let bib_tex_content = r#"field-name = "test\"\\%\""#;
        let mut tokenizer = Tokenizer::new(bib_tex_content.chars());
        let mut parser = FieldParser::new(&mut tokenizer);
        let (field_name, content) = parser.field();
        assert!(tokenizer.eof);
        assert_eq!("field-name", field_name.as_str());
        assert_eq!("test\\\"\\\\%\\\"", content.as_str());
    }
}