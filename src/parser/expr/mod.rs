use crate::parser::r#type::ValueType;
use crate::tokenizer::token::{Literal, Operator};

#[derive(Debug)]
pub enum Expression {
    NumberLiteral {
        value: Literal,
    },
    IdentifierLiteral {
        value: Literal,
        type_: Option<ValueType>,
    },
    CharLiteral {
        value: Literal,
    },
    Operation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
        type_: Option<ValueType>,
    },
    Array {
        content: Vec<Expression>,
    },
    Deref {
        value: Box<Expression>
    },
    Access {
        value: Box<Expression>
    }
}