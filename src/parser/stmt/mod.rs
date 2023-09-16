use crate::parser::expr::Expression;
use crate::tokenizer::token::{Literal, TypeType};

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Literal,
        type_: TypeType,
        expression: Option<Expression>
    },
    Assign {
        identifier: Literal,
        expression: Expression,
    },
    Exit {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
}