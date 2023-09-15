use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;
use crate::tokenizer::token::{Keyword, Literal, LiteralType, Operator, Token, TypeType};

pub mod token;

lazy_static! {
    static ref IGNORABLE: [char; 3] = [' ', '\n', '\r'];
    static ref LITERAL_REGEX: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
}

pub struct Tokenizer {
    input: Vec<char>,
}

impl Tokenizer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];


        loop {
            let token = self.next_token().unwrap();
            if token == Token::EOF { break; }
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_ignorable();
        if self.input.is_empty() { return Ok(Token::EOF); }

        let token = unsafe {
            match self.input.get_unchecked(0) {
                &',' | &':' | &';' | &'+' | &'-' | &'*' | &'/' | &'&' | &'=' => self.tokenize_singe_symbol(),
                c if LITERAL_REGEX.is_match(&c.to_string()) => self.tokenize_identifier(),
                c if NUMBER_REGEX.is_match(&c.to_string()) => self.tokenize_number(),
                &'\'' => self.tokenize_char(),
                &'"' => self.tokenize_string(),
                c => bail!("Unknown char '{}'!", c)
            }
        };

        Ok(token)
    }

    fn tokenize_singe_symbol(&mut self) -> Token {
        let c = self.input.remove(0);
        return match c {
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '+' | '-' | '*' | '/' | '&' | '=' => Token::Operation {
                operator: match c {
                    '+' => Operator::Plus,
                    '-' => Operator::Minus,
                    '*' => Operator::Times,
                    '/' => Operator::Divide,
                    '&' => Operator::And,
                    '=' => Operator::Assign,
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };
    }

    fn tokenize_identifier(&mut self) -> Token {
        let literal: Literal = self.read_matching(|c| LITERAL_REGEX.is_match(&c.to_string())).iter()
            .map(|c| *c as u8)
            .collect();

        return match literal.as_slice() {
            b"exit" | b"let" | b"print" => Token::Keyword {
                keyword: match literal.as_slice() {
                    b"let" => Keyword::Let,
                    b"exit" => Keyword::Exit,
                    b"print" => Keyword::Print,
                    _ => unreachable!()
                }
            },
            b"u64" | b"char" => Token::Type {
                type_: match literal.as_slice() {
                    b"u64" => TypeType::U64,
                    b"char" => TypeType::Char,
                    _ => unreachable!()
                }
            },
            _ => Token::Literal {
                value: literal,
                type_: LiteralType::Identifier,
            }
        };
    }

    fn tokenize_number(&mut self) -> Token {
        let literal: Literal = self.read_matching(|c| NUMBER_REGEX.is_match(&c.to_string())).iter()
            .map(|c| *c as u8)
            .collect();

        return Token::Literal {
            value: literal,
            type_: LiteralType::Number,
        };
    }

    fn tokenize_char(&mut self) -> Token {
        self.input.remove(0);
        let literal: Literal = self.read_matching(|c| c != &'\'').iter()
            .map(|c| *c as u8)
            .collect();
        self.input.remove(0);

        return Token::Literal {
            value: literal,
            type_: LiteralType::Char,
        };
    }

    fn tokenize_string(&mut self) -> Token {
        self.input.remove(0);
        let literal: Literal = self.read_matching(|c| c != &'"').iter()
            .map(|c| *c as u8)
            .collect();
        self.input.remove(0);

        Token::Literal {
            value: literal,
            type_: LiteralType::String,
        }
    }

    fn read_matching(&mut self, predicate: fn(&char) -> bool) -> Vec<char> {
        let mut buffer: Vec<char> = vec![];

        unsafe {
            while predicate(&self.input.get_unchecked(0)) {
                buffer.push(self.input.remove(0));
            }
        }

        return buffer;
    }

    fn skip_ignorable(&mut self) {
        unsafe {
            while !self.input.is_empty() && IGNORABLE.contains(self.input.get_unchecked(0)) {
                self.input.remove(0);
            }
        }
    }
}
