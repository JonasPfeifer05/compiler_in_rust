use crate::parser::expr::Expression;
use crate::tokenizer::token::Literal;

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Literal,
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