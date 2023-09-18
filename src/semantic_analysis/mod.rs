pub mod symbol_table;

use std::mem;

use crate::parser::expr::Expression;
use crate::parser::r#type::ValueType;
use crate::semantic_analysis::symbol_table::SymbolTable;
use crate::tokenizer::token::Operator;

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
                if let ValueType::U64 = index.get_type() {} else { None.unwrap() }
            }
            Expression::IdentifierLiteral { value, type_, .. } => {
                type_.replace(symbol_table.get(value).clone());
            }
            Expression::Operation { rhs, lhs, operator, type_ } => {
                rhs.resolve(symbol_table);
                lhs.resolve(symbol_table);

                if rhs.get_type().is_pointer() {
                    let _ = mem::replace(rhs, Box::new(Expression::Deref { value: rhs.clone() }));
                }
                if lhs.get_type().is_pointer() {
                    let _ = mem::replace(lhs, Box::new(Expression::Deref { value: lhs.clone() }));
                }

                type_.replace(operator.get_result_type(&lhs.get_type(), &rhs.get_type()));
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
        }
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            Expression::NumberLiteral { .. } => ValueType::U64,
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
            Expression::Reference { reference } => ValueType::Pointer { points_to: Box::new(reference.get_type()) }
        }
    }
}

impl ValueType {
    pub fn is_pointer(&self) -> bool {
        match self {
            ValueType::Pointer { .. } => true,
            ValueType::U64 |
            ValueType::Char |
            ValueType::Array { .. } => false,
        }
    }
}

impl Operator {
    pub fn get_result_type(&self, lhs: &ValueType, rhs: &ValueType) -> ValueType {
        match self {
            Operator::Plus => Self::plus(lhs, rhs),
            Operator::Minus => Self::minus(lhs, rhs),
            Operator::Times => Self::times(lhs, rhs),
            Operator::Divide => Self::divide(lhs, rhs),
            _ => unreachable!()
        }
    }

    fn plus(lhs: &ValueType, rhs: &ValueType) -> ValueType {
        match lhs {
            ValueType::U64 => match rhs {
                ValueType::U64 => ValueType::U64,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }

    fn minus(lhs: &ValueType, rhs: &ValueType) -> ValueType {
        match lhs {
            ValueType::U64 => match rhs {
                ValueType::U64 => ValueType::U64,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }

    fn times(lhs: &ValueType, rhs: &ValueType) -> ValueType {
        match lhs {
            ValueType::U64 => match rhs {
                ValueType::U64 => ValueType::U64,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }

    fn divide(lhs: &ValueType, rhs: &ValueType) -> ValueType {
        match lhs {
            ValueType::U64 => match rhs {
                ValueType::U64 => ValueType::U64,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }
}