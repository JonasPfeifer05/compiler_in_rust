use std::iter::Peekable;
use std::str::Chars;
use anyhow::bail;
use lazy_static::lazy_static;
use regex::Regex;
use crate::tokenizer::token::{Keyword, Literal, LiteralType, Operator, Token, TypeType};

pub mod token;

lazy_static! {
    static ref IGNORABLE: [char; 3] = [' ', '\n', '\r'];
    static ref LITERAL_START_REGEX: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
    static ref LITERAL_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: Peekable<Chars<'a>>) -> Self {
        Self {
            input,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            match self.next_token() {
                Ok(Token::EOF) => break,
                Ok(token) => tokens.push(token),
                Err(error) => {
                    eprintln!("ERROR WHILE TOKENIZING: {}", error);
                    std::process::exit(1);
                }
            }
        }

        tokens
    }

    fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_ignorables();
        if self.input.peek().is_none() { return Ok(Token::EOF); }

        match self.input.peek().expect("THIS WILL NEVER OCCUR!") {
            &',' | &':' | &';' | &'+' | &'-' | &'*' | &'/' | &'&' | &'=' | &'(' | &')' | &'[' | &']' => self.tokenize_singe_symbol(),
            character if LITERAL_START_REGEX.is_match(&character.to_string()) => self.tokenize_identifier(),
            character if NUMBER_REGEX.is_match(&character.to_string()) => self.tokenize_number(),
            &'\'' => self.tokenize_char(),
            &'"' => self.tokenize_string(),
            c => bail!("Unknown character '{}'!", c)
        }
    }

    fn tokenize_singe_symbol(&mut self) -> anyhow::Result<Token> {
        let char = self.consume_char()?;
        Ok(
            match char {
                ',' => Token::Comma,
                ':' => Token::Colon,
                ';' => Token::Semicolon,
                '+' | '-' | '*' | '/' | '&' | '=' => Token::Operation {
                    operator: match char {
                        '+' => Operator::Plus,
                        '-' => Operator::Minus,
                        '*' => Operator::Times,
                        '/' => Operator::Divide,
                        '&' => Operator::And,
                        '=' => Operator::Assign,
                        _ => unreachable!()
                    }
                },
                '(' => Token::OpenParent,
                ')' => Token::ClosedParent,
                '[' => Token::OpenBracket,
                ']' => Token::ClosedBracket,
                _ => bail!("Unknown char encountered!")
            }
        )
    }

    fn tokenize_identifier(&mut self) -> anyhow::Result<Token> {
        let literal: Literal = self.read_matching(|c| LITERAL_REGEX.is_match(&c.to_string())).iter()
            .map(|c| *c as u8)
            .collect();

        Ok(
            match literal.as_slice() {
                b"exit" | b"let" | b"print" => Token::Keyword {
                    keyword: match literal.as_slice() {
                        b"let" => Keyword::Let,
                        b"exit" => Keyword::Exit,
                        b"print" => Keyword::Print,
                        _ => unreachable!()
                    }
                },
                b"u64" | b"u32" | b"u16" | b"u8" | b"char" => Token::Type {
                    type_: match literal.as_slice() {
                        b"u64" => TypeType::U64,
                        b"u32" => TypeType::U32,
                        b"u16" => TypeType::U16,
                        b"u8" => TypeType::U8,
                        b"char" => TypeType::Char,
                        _ => unreachable!()
                    }
                },
                _ => Token::Literal {
                    value: literal,
                    type_: LiteralType::Identifier,
                }
            }
        )
    }

    fn tokenize_number(&mut self) -> anyhow::Result<Token> {
        let literal: Literal = self.read_matching(|c| NUMBER_REGEX.is_match(&c.to_string())).iter()
            .map(|c| *c as u8)
            .collect();

        Ok(
            Token::Literal {
                value: literal,
                type_: LiteralType::Number,
            }
        )
    }

    fn tokenize_char(&mut self) -> anyhow::Result<Token> {
        // Get rid of leading '
        self.consume_char()?;

        let literal: Literal = self.read_matching(|c| c != &'\'').iter()
            .map(|c| *c as u8)
            .collect();

        if self.consume_char()? != '\'' { bail!("Char didnt end with \"'\"!") }

        Ok(
            Token::Literal {
                value: literal,
                type_: LiteralType::Char,
            }
        )
    }

    fn tokenize_string(&mut self) -> anyhow::Result<Token> {
        // Get rid of leading "
        self.consume_char()?;

        let literal: Literal = self.read_matching(|c| c != &'"').iter()
            .map(|c| *c as u8)
            .collect();

        if self.consume_char()? != '"' { bail!("String didnt end with '\"'!") }

        Ok(
            Token::Literal {
                value: literal,
                type_: LiteralType::String,
            }
        )
    }

    fn read_matching(&mut self, predicate: fn(&char) -> bool) -> Vec<char> {
        let mut buffer: Vec<char> = vec![];
        while self.input.peek().is_some() && predicate(self.input.peek().expect("THIS WILL NEVER OCCUR!")) {
            buffer.push(self.consume_char().expect("THIS WILL NEVER OCCUR!"));
        }
        return buffer;
    }

    fn skip_ignorables(&mut self) {
        while self.input.peek().is_some() && IGNORABLE.contains(self.input.peek().expect("THIS WILL NEVER OCCUR!")) {
            self.consume_char().expect("THIS WILL NEVER OCCUR!");
        }
    }

    fn consume_char(&mut self) -> anyhow::Result<char> {
        if self.input.peek().is_none() { bail!("Tried to consume char but ran out of data!") }
        return Ok(self.input.next().expect("THIS WILL NEVER OCCUR!"));
    }
}
