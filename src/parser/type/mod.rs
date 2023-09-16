use std::fmt::{Display, Formatter, Write};
use crate::tokenizer::token::{Literal, literal_to_string};

#[derive(Eq, PartialEq, Debug)]
pub enum ValueType {
    U64,
    Char,
    Pointer { points_to: Box<Self> },
    Array { content_type: Box<Self>, len: Literal }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::U64 => f.write_str("u64"),
            ValueType::Char => f.write_str("char"),
            ValueType::Pointer { points_to } => {
                f.write_char('&').unwrap();
                f.write_str(&points_to.to_string())
            }
            ValueType::Array { content_type, len } => {
                f.write_char('[').unwrap();
                f.write_str(&content_type.to_string()).unwrap();
                f.write_str(", ").unwrap();
                f.write_str(&literal_to_string(len)).unwrap();
                f.write_char(']')
            }
        }
    }
}