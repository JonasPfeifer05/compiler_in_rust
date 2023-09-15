use crate::parser::expr::Expression;
use crate::tokenizer::token::Literal;

pub enum Statement {
    Let {
        identifier: Literal,
        value: Option<Expression>
    },
    Assign {
        identifier: Literal,
        value: Expression,
    },
    Exit {
        value: Expression,
    },
    Print {
        value: Expression,
    },
}