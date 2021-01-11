use crate::tokens::TokenType::{
    And, Bang, BangEqual, Class, Comma, Dot, Else, Eof, Equal, EqualEqual, False, For, Fun,
    Greater, GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil,
    Number, Or, Plus, Print, Return, RightBrace, RightParen, Semicolon, Slash, Star, String_,
    Super, This, True, Var, While,
};
use crate::tokens::{Literal, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::str::FromStr;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_owned(), And);
        m.insert("class".to_owned(), Class);
        m.insert("else".to_owned(), Else);
        m.insert("false".to_owned(), False);
        m.insert("for".to_owned(), For);
        m.insert("fun".to_owned(), Fun);
        m.insert("if".to_owned(), If);
        m.insert("nil".to_owned(), Nil);
        m.insert("or".to_owned(), Or);
        m.insert("print".to_owned(), Print);
        m.insert("return".to_owned(), Return);
        m.insert("super".to_owned(), Super);
        m.insert("this".to_owned(), This);
        m.insert("true".to_owned(), True);
        m.insert("var".to_owned(), Var);
        m.insert("while".to_owned(), While);
        m
    };
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: NonZeroUsize,
}

impl Scanner {
    pub const fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: NonZeroUsize::new(1).unwrap(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(Eof, "", None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let type_ = if self.matches('=') { BangEqual } else { Bang };
                self.add_token(type_)
            }
            '=' => {
                let type_ = if self.matches('=') { EqualEqual } else { Equal };
                self.add_token(type_)
            }
            '<' => {
                let type_ = if self.matches('=') { LessEqual } else { Less };
                self.add_token(type_)
            }
            '>' => {
                let type_ = if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(type_)
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.increment_line(),
            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number()
                } else if is_alphanumeric(c) {
                    self.identifier()
                } else {
                    crate::error(self.line, "Unexpected character")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_full_token(type_, None);
    }

    fn add_full_token(&mut self, type_: TokenType, literal: Option<Literal>) {
        let token = Token::new(
            type_,
            &self.source[self.start..self.current],
            literal,
            self.line,
        );
        self.tokens.push(token);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return true;
        }
        if expected == self.source.chars().nth(self.current).unwrap() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.increment_line();
            }
            self.advance();
        }
        if self.is_at_end() {
            crate::error(self.line, "Unterminated string.");
        }
        self.advance();
        let literal =
            Literal::String_(self.source[(self.start + 1)..(self.current - 1)].to_owned());
        self.add_full_token(String_, Some(literal));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal = Literal::Number(
            f64::from_str(&self.source[(self.start + 1)..(self.current - 1)]).unwrap(),
        );
        self.add_full_token(Number, Some(literal));
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[(self.start + 1)..(self.current - 1)];
        let type_ = KEYWORDS
            .get(text)
            .map_or_else(|| Identifier, std::clone::Clone::clone);
        self.add_token(type_);
    }

    fn increment_line(&mut self) {
        self.line = NonZeroUsize::new(self.line.get() + 1).unwrap();
    }
}

const fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
