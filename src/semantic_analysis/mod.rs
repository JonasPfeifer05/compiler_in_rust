pub mod symbol_table;

use std::mem;

use crate::parser::expr::Expression;
use crate::parser::r#type::{CastVariant, ValueType};
use crate::semantic_analysis::symbol_table::SymbolTable;

impl Expression {
    pub fn resolve(&mut self, symbol_table: &SymbolTable) {
        match self {
            Expression::NumberLiteral { .. } |
            Expression::CharLiteral { .. } => {}
            Expression::Deref { value } => {
                value.resolve(symbol_table);
                if let ValueType::Pointer { .. } = value.get_type() {} else { None.unwrap() }
            }
            Expression::Access { value, index } => {
                value.resolve(symbol_table);
                if let ValueType::Pointer { .. } = value.get_type() {} else if let ValueType::Array { .. } = value.get_type() {} else { None.unwrap() }

                index.resolve(symbol_table);
                let index_type = index.get_type();
                if index_type != ValueType::U64 {
                    let _ = index_type.get_casts().get(&ValueType::U64).unwrap();
                    let _ = mem::replace(index, Box::new(Expression::Cast { value: index.clone(), to: ValueType::U64 }));
                    index.resolve(symbol_table)
                }
                if let ValueType::U64 = index.get_type() {} else { None.unwrap() }
            }
            Expression::IdentifierLiteral { value, type_, .. } => {
                type_.replace(symbol_table.get(value).clone());
            }
            Expression::Operation { rhs, lhs, operator, type_ } => {
                lhs.resolve(symbol_table);
                if lhs.get_type().is_pointer() {
                    let _ = mem::replace(lhs, Box::new(Expression::Deref { value: lhs.clone() }));
                    lhs.resolve(symbol_table)
                }

                rhs.resolve(symbol_table);
                if rhs.get_type().is_pointer() {
                    let _ = mem::replace(rhs, Box::new(Expression::Deref { value: rhs.clone() }));
                    rhs.resolve(symbol_table)
                }


                let left_operator_type_to_result_types = lhs.get_type().get_operation_results(operator);

                let left_operation_result = left_operator_type_to_result_types.get(&rhs.get_type());

                if left_operation_result.is_none() {
                    let cast_to_types: Vec<_> = rhs.get_type().get_casts().iter()
                        .filter(|entry| entry.1 == &CastVariant::Explicit)
                        .map(|entry| entry.0)
                        .cloned()
                        .collect();

                    let mut cast_to = None;
                    for valid_operator_type in left_operator_type_to_result_types.keys() {
                        if cast_to_types.contains(valid_operator_type) {
                            cast_to = Some(valid_operator_type);
                            break;
                        }
                    }
                    if let Some(cast_to) = cast_to {
                        let _ = mem::replace(rhs, Box::new(Expression::Cast { value: rhs.clone(), to: cast_to.clone() }));
                        rhs.resolve(symbol_table)
                    }
                }

                let right_operator_type_to_result_types = rhs.get_type().get_operation_results(operator);

                let right_operation_result = right_operator_type_to_result_types.get(&lhs.get_type());

                if right_operation_result.is_none() {
                    let cast_to_types: Vec<_> = lhs.get_type().get_casts().iter()
                        .filter(|entry| entry.1 == &CastVariant::Explicit)
                        .map(|entry| entry.0)
                        .cloned()
                        .collect();

                    let mut cast_to = None;
                    for valid_operator_type in right_operator_type_to_result_types.keys() {
                        if cast_to_types.contains(valid_operator_type) {
                            cast_to = Some(valid_operator_type);
                            break;
                        }
                    }
                    if let Some(cast_to) = cast_to {
                        let _ = mem::replace(lhs, Box::new(Expression::Cast { value: lhs.clone(), to: cast_to.clone() }));
                        lhs.resolve(symbol_table)
                    } else { None::<()>.unwrap(); }
                }

                type_.replace(right_operator_type_to_result_types.get(&rhs.get_type()).unwrap().clone());
            }
            Expression::Array { content } => {
                let mut last_type = None;
                for expression in content {
                    expression.resolve(symbol_table);
                    let type_ = expression.get_type();
                    if let Some(last_type) = &last_type {
                        if &type_ != last_type { None.unwrap() }
                    }
                    last_type = Some(type_);
                }
            }
            Expression::Reference { reference: to_reference } => {
                to_reference.resolve(symbol_table);
                if let Expression::IdentifierLiteral { .. } = to_reference.as_ref() {} else if let Expression::Access { .. } = to_reference.as_ref() {} else { None.unwrap() }
            }
            Expression::Cast { value, to } => {
                value.resolve(symbol_table);

                let allowed_casts = value.get_type().get_casts();
                let _ = allowed_casts.get(to).unwrap();
            }
        }
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            Expression::NumberLiteral { internal_type, .. } => internal_type.clone(),
            Expression::IdentifierLiteral { type_, .. } => type_.clone().unwrap(),
            Expression::CharLiteral { .. } => ValueType::Char,
            Expression::Operation { type_, .. } => type_.clone().unwrap(),
            Expression::Array { content } => ValueType::Array { content_type: Box::new(content.first().unwrap().get_type()), len: content.len() },
            Expression::Deref { value } => {
                if let ValueType::Pointer { points_to } = value.get_type() { *points_to } else { None.unwrap() }
            }
            Expression::Access { value, .. } => {
                match value.get_type() {
                    ValueType::Pointer { points_to } => match points_to.as_ref() {
                        ValueType::Array { content_type, .. } => *content_type.clone(),
                        _ => *points_to.clone(),
                    },
                    ValueType::Array { content_type, .. } => *content_type,
                    _ => unreachable!()
                }
            }
            Expression::Reference { reference } => ValueType::Pointer { points_to: Box::new(reference.get_type()) },
            Expression::Cast { to, .. } => to.clone(),
        }
    }
}

impl ValueType {
    pub fn is_pointer(&self) -> bool {
        match self {
            ValueType::Pointer { .. } => true,
            ValueType::U64 |
            ValueType::U32 |
            ValueType::U16 |
            ValueType::U8 |
            ValueType::Char |
            ValueType::Array { .. } => false,
        }
    }
}