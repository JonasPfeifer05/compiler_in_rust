use std::fmt::{Display, Formatter, Write};
use std::mem;
use serde::{Deserialize, Serialize};

pub type Literal = Vec<u8>;

fn literal_to_string(literal: &Literal) -> String {
    String::from_utf8_lossy(literal.as_slice()).parse().unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Token {
    EOF,
    Ignored,

    Keyword { literal: Literal },

    Literal {
        type_: LiteralType,
        value: Literal,
    },

    Type {
        type_: TypeType
    },

    Operation {
        operator: Operator
    },

    Comma,
    Colon,
    Semicolon,

    OpenParent,
    ClosedParent,

    OpenBracket,
    ClosedBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOF => f.write_str("eof"),
            Token::Ignored => f.write_str(""),
            Token::Keyword { literal } => {
                f.write_str("keyword ").unwrap();
                f.write_str(&literal_to_string(literal))
            },
            Token::Literal { value, type_ } => {
                f.write_str(&type_.to_string()).unwrap();
                f.write_char(' ').unwrap();
                f.write_str(&literal_to_string(value))
            }
            Token::Type { type_ } => f.write_str(&type_.to_string()),
            Token::Operation { operator } => f.write_str(&operator.to_string()),
            Token::Comma => f.write_char(','),
            Token::Colon => f.write_char(':'),
            Token::Semicolon => f.write_char(';'),
            Token::OpenParent => f.write_char('('),
            Token::ClosedParent => f.write_char(')'),
            Token::OpenBracket => f.write_char('['),
            Token::ClosedBracket => f.write_char(']'),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum LiteralType {
    String,
    Char,
    Number,
    Identifier,
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::String => f.write_str("string"),
            LiteralType::Char => f.write_str("char"),
            LiteralType::Number => f.write_str("number"),
            LiteralType::Identifier => f.write_str("ident"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeType {
    U64,
    Char,
}

impl Display for TypeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeType::U64 => f.write_str("u64"),
            TypeType::Char => f.write_str("char"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Assign,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Plus => f.write_char('+'),
            Operator::Minus => f.write_char('-'),
            Operator::Times => f.write_char('*'),
            Operator::Divide => f.write_char('/'),
            Operator::And => f.write_char('&'),
            Operator::Assign => f.write_char('='),
        }
    }
}

pub fn equals_variant<T>(t1: &T, t2: &T) -> bool {
    mem::discriminant(t1) == mem::discriminant(t2)
}