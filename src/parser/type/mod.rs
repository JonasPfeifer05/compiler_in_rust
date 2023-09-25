use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use crate::tokenizer::token::Operator;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ValueType {
    U64,
    U32,
    U16,
    U8,
    Char,
    Pointer { points_to: Box<Self> },
    Array { content_type: Box<Self>, len: usize },
}

#[derive(Eq, PartialEq)]
pub enum CastVariant {
    Explicit,
    Implicit,
}

impl ValueType {
    pub fn get_casts(&self) -> HashMap<ValueType, CastVariant> {
        match self {
            ValueType::U64 => collection!(
                ValueType::U64 => CastVariant::Explicit,
                ValueType::U32 => CastVariant::Implicit,
                ValueType::U16 => CastVariant::Implicit,
                ValueType::U8 => CastVariant::Implicit,

                ValueType::Char => CastVariant::Implicit,
            ),
            ValueType::U32 => collection!(
                ValueType::U64 => CastVariant::Explicit,
                ValueType::U32 => CastVariant::Explicit,
                ValueType::U16 => CastVariant::Implicit,
                ValueType::U8 => CastVariant::Implicit,

                ValueType::Char => CastVariant::Implicit,
            ),
            ValueType::U16 => collection!(
                ValueType::U64 => CastVariant::Explicit,
                ValueType::U32 => CastVariant::Explicit,
                ValueType::U16 => CastVariant::Explicit,
                ValueType::U8 => CastVariant::Implicit,

                ValueType::Char => CastVariant::Implicit,
            ),
            ValueType::U8 => collection!(
                ValueType::U64 => CastVariant::Explicit,
                ValueType::U32 => CastVariant::Explicit,
                ValueType::U16 => CastVariant::Explicit,
                ValueType::U8 => CastVariant::Explicit,
                ValueType::Char => CastVariant::Explicit,
            ),
            ValueType::Char => collection!(
                ValueType::Char => CastVariant::Explicit,

                ValueType::U8 => CastVariant::Explicit,
                ValueType::U16 => CastVariant::Explicit,
                ValueType::U32 => CastVariant::Explicit,
                ValueType::U64 => CastVariant::Explicit,
            ),
            ValueType::Pointer { .. } => collection!(
                self.clone() => CastVariant::Explicit,
            ),
            ValueType::Array { .. } => collection!(
                self.clone() => CastVariant::Explicit,
            ),
        }
    }
}

pub struct OperationResult {
    pub input: ValueType,
    pub output: ValueType,
}

impl ValueType {
    pub fn get_operation_results(&self, operator: &Operator) -> HashMap<ValueType, ValueType> {
        match self {
            ValueType::U64 => match operator {
                Operator::Plus => collection!(ValueType::U64 => ValueType::U64),
                Operator::Minus => collection!(ValueType::U64 => ValueType::U64),
                Operator::Times => collection!(ValueType::U64 => ValueType::U64),
                Operator::Divide => collection!(ValueType::U64 => ValueType::U64),
                _ => collection!()
            },
            ValueType::U32 => match operator {
                Operator::Plus => collection!(ValueType::U32 => ValueType::U32),
                Operator::Minus => collection!(ValueType::U32 => ValueType::U32),
                Operator::Times => collection!(ValueType::U32 => ValueType::U32),
                Operator::Divide => collection!(ValueType::U32 => ValueType::U32),
                _ => collection!()
            },
            ValueType::U16 => match operator {
                Operator::Plus => collection!(ValueType::U16 => ValueType::U16),
                Operator::Minus => collection!(ValueType::U16 => ValueType::U16),
                Operator::Times => collection!(ValueType::U16 => ValueType::U16),
                Operator::Divide => collection!(ValueType::U16 => ValueType::U16),
                _ => collection!()
            },
            ValueType::U8 => match operator {
                Operator::Plus => collection!(ValueType::U8 => ValueType::U8),
                Operator::Minus => collection!(ValueType::U8 => ValueType::U8),
                Operator::Times => collection!(ValueType::U8 => ValueType::U8),
                Operator::Divide => collection!(ValueType::U8 => ValueType::U8),
                _ => collection!()
            },
            ValueType::Char => collection!(),
            ValueType::Pointer { .. } => collection!(),
            ValueType::Array { .. } => collection!(),
        }
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::U64 => f.write_str("u64"),
            ValueType::U32 => f.write_str("u32"),
            ValueType::U16 => f.write_str("u16"),
            ValueType::U8 => f.write_str("u8"),
            ValueType::Char => f.write_str("char"),
            ValueType::Pointer { points_to } => {
                f.write_char('&').unwrap();
                f.write_str(&points_to.to_string())
            }
            ValueType::Array { content_type, len } => {
                f.write_char('[').unwrap();
                f.write_str(&content_type.to_string()).unwrap();
                f.write_str(", ").unwrap();
                f.write_str(&len.to_string()).unwrap();
                f.write_char(']')
            }
        }
    }
}