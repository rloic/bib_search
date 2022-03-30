use std::str::Chars;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Info {
    pub line: usize,
    pub column: usize,
}

pub struct Tokenizer<'r> {
    pub iterator: Chars<'r>,
    pub lookahead: char,
    pub skip_ws: bool,
    pub skip_comment: bool,
    pub eof: bool,
    pub info: Info,
}

impl<'r> Tokenizer<'r> {
    pub fn new(iterator: Chars<'r>) -> Tokenizer<'r> {
        let mut tokenizer = Tokenizer {
            iterator,
            lookahead: ' ',
            skip_ws: true,
            skip_comment: true,
            eof: false,
            info: Info {
                line: 1,
                column: 0,
            },
        };

        tokenizer.skip(&ANY);

        tokenizer
    }

    #[allow(non_snake_case)]
    fn __NEXT_CHAR__(&mut self) {
        if let Some(symbol) = self.iterator.next() {
            self.lookahead = symbol;
            if NEW_LINE(self.lookahead) {
                self.info.line += 1;
                self.info.column = 1;
            } else {
                self.info.column += 1;
            }
        } else {
            self.eof = true;
        }
    }

    fn ignore_ws(&mut self) {
        while !self.eof && self.lookahead.is_whitespace() {
            self.__NEXT_CHAR__();
        }
    }

    fn ignore_comment(&mut self) {
        while !self.eof && COMMENT(self.lookahead) {
            self.__NEXT_CHAR__();
            while !self.eof && !NEW_LINE(self.lookahead) {
                self.__NEXT_CHAR__();
            }
            self.__NEXT_CHAR__();
            self.ignore_ws();
        }
    }

    pub fn skip<P: Fn(char) -> bool>(&mut self, p: &'static P) {
        if !p(self.lookahead) {
            panic!("Unexpected symbol '{}' at {}, {}",
                   self.lookahead,
                   self.info.line,
                   self.info.column
            );
        }
        self.__NEXT_CHAR__();
        if self.skip_ws { self.ignore_ws(); }
        if self.skip_comment { self.ignore_comment(); }
        if self.skip_ws { self.ignore_ws(); }
    }

    pub fn skip_optional<P: Fn(char) -> bool>(&mut self, p: &'static P) {
        if p(self.lookahead) { self.skip(&ANY); }
    }

    pub fn skip_while<P: Fn(char) -> bool>(&mut self, p: &'static P) {
        while !self.eof && p(self.lookahead) {
            self.skip(&ANY);
        }
    }

    pub fn consume<P: Fn(char) -> bool>(&mut self, p: &'static P) -> char {
        let result = self.lookahead;
        self.skip(p);
        result
    }

    pub fn consume_while<P: Fn(char) -> bool>(&mut self, p: &'static P) -> String {
        let mut text = String::new();
        while !self.eof && p(self.lookahead) {
            text.push(self.consume(&ANY));
        }
        text
    }
}

#[allow(non_snake_case)]
pub fn ANY(_: char) -> bool { true }

#[allow(non_snake_case)]
pub fn AT_SIGN(c: char) -> bool { c == '@' }

#[allow(non_snake_case)]
pub fn ASSIGN(c: char) -> bool { c == '=' }

#[allow(non_snake_case)]
pub fn OPEN_BRACKET(c: char) -> bool { c == '{' }

#[allow(non_snake_case)]
pub fn CLOSE_BRACKET(c: char) -> bool { c == '}' }

#[allow(non_snake_case)]
pub fn COMMA(c: char) -> bool { c == ',' }

#[allow(non_snake_case)]
pub fn DOUBLE_QUOTE(c: char) -> bool { c == '"' }

#[allow(non_snake_case)]
pub fn CONCAT(c: char) -> bool { c == '#' }

#[allow(non_snake_case)]
pub fn DIGIT(c: char) -> bool {
    c.is_digit(10)
}

#[allow(non_snake_case)]
pub fn ALPHA(c: char) -> bool { c.is_alphabetic() }

#[allow(non_snake_case)]
pub fn ESCAPED(c: char) -> bool { c == '\\' }

#[allow(non_snake_case)]
pub fn NEW_LINE(c: char) -> bool { c == '\n' }

#[allow(non_snake_case)]
pub fn WS(c: char) -> bool { c.is_whitespace() }

#[allow(non_snake_case)]
pub fn COMMENT(c: char) -> bool { c == '%' }

