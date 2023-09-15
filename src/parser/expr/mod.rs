use crate::tokenizer::token::{Literal, Operator};

pub enum Expression {
    NumberLiteral {
        value: Literal,
    },
    IdentifierLiteral {
        value: Literal,
    },
    CharLiteral {
        value: Literal,
    },
    Operation {
        lhs: Box<Expression>,
        operator: Operator,
        rhs: Box<Expression>,
    },
    Array {
        content: Vec<Expression>
    },
    Deref {
        value: Box<Expression>
    },
    Access {
        value: Box<Expression>
    }
}