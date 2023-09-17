use std::ops::Deref;
use std::process::id;
use crate::parser::expr::Expression;
use crate::parser::r#type::ValueType;
use crate::semantic_analysis::symbol_table::SymbolTable;
use crate::tokenizer::token::Literal;

#[derive(Debug)]
pub enum Statement {
    Let {
        identifier: Literal,
        type_: ValueType,
        expression: Option<Expression>,
    },
    Assign {
        assignee: Expression,
        expression: Expression,
    },
    Exit {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
}

impl Statement {
    pub fn resolve(&mut self, symbol_table: &mut SymbolTable) {
        match self {
            Statement::Let { type_, expression, identifier } => {
                if let Some(expression) = expression {
                    expression.resolve(symbol_table);
                    if type_ != &expression.get_type() { None.unwrap() }
                }
                symbol_table.register(identifier.clone(), type_.clone());
            }
            Statement::Assign { assignee, expression } => {
                assignee.resolve(symbol_table);
                let assign_type = if let ValueType::Pointer { points_to } = assignee.get_type() { points_to } else { None.unwrap() };
                expression.resolve(symbol_table);
                if assign_type.as_ref() != &expression.get_type() { None.unwrap() }
            }
            Statement::Exit { .. } => {}
            Statement::Print { .. } => {}
        }
    }
}