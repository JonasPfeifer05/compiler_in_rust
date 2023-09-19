use crate::parser::r#type::ValueType;
use crate::tokenizer::token::{Literal, Operator};

#[derive(Debug, Clone)]
pub enum Expression {
    NumberLiteral {
        value: Literal,
        internal_type: ValueType,
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
    Reference {
        reference: Box<Expression>,
    },
    Deref {
        value: Box<Expression>
    },
    Access {
        value: Box<Expression>,
        index: Box<Expression>,
    },
    Cast { value: Box<Expression>, to: ValueType },
}