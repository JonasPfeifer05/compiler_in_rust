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
        println!("{:?}", self);
        match self {
            Statement::Let { type_, expression, identifier } => {
                if let Some(expression) = expression {
                    expression.resolve(symbol_table);
                    let expression_type = expression.get_type();
                    if type_ != &expression_type {
                        let _ = expression_type.get_casts().get(type_).unwrap();
                        let _ = std::mem::replace(expression, Expression::Cast { value: Box::new(expression.clone()), to: type_.clone() });
                        expression.resolve(symbol_table);
                    }
                }
                symbol_table.register(identifier.clone(), type_.clone());
            }
            Statement::Assign { assignee, expression } => {
                assignee.resolve(symbol_table);
                let assignee_type = match assignee.get_type() {
                    ValueType::Pointer { points_to } => *points_to,
                    _ => {
                        match assignee {
                            Expression::IdentifierLiteral { .. } => {
                                let stored_type = assignee.get_type();
                                let _ = std::mem::replace(assignee, Expression::Reference { reference: Box::new(assignee.clone()) });
                                assignee.resolve(symbol_table);
                                stored_type
                            }
                            Expression::Access { .. } => {
                                let stored_type = assignee.get_type();
                                let _ = std::mem::replace(assignee, Expression::Reference { reference: Box::new(assignee.clone()) });
                                assignee.resolve(symbol_table);
                                stored_type

                            }
                            _ => unreachable!(),
                        }
                    }
                };
                expression.resolve(symbol_table);
                let expression_type = expression.get_type();
                if assignee_type != expression_type {
                    let _ = expression_type.get_casts().get(&assignee_type).unwrap();
                    let _ = std::mem::replace(expression, Expression::Cast { value: Box::new(expression.clone()), to: assignee_type });
                    expression.resolve(symbol_table);
                }
            }
            Statement::Exit { .. } => {}
            Statement::Print { .. } => {}
        }
    }
}