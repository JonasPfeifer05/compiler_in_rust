use crate::parser::expr::Expression;
use crate::parser::r#type::ValueType;
use crate::parser::stmt::Statement;
use crate::tokenizer::token::{Keyword, Literal, LiteralType, Operator, Token, TypeType};

pub mod expr;
pub mod stmt;
pub mod r#type;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse_statements(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while !self.tokens.is_empty() {
            statements.push(self.parse_statement().unwrap())
        }

        statements
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword { keyword }) = self.get_keyword() {
            match keyword {
                Keyword::Let => self.parse_let(),
                Keyword::Exit => self.parse_exit(),
                Keyword::Print => self.parse_print(),
            }
        } else if let Token::Literal {
            type_: LiteralType::Identifier,
            value
        } = self.tokens.remove(0) {
            self.parse_assign(value)
        } else {
            None
        }
    }

    fn parse_let(&mut self) -> Option<Statement> {
        let identifier = if let Some(Token::Literal { type_: LiteralType::Identifier, value }) = self.get_literal() {
            value
        } else {
            return None;
        };
        if let Token::Colon = &self.tokens.remove(0) {} else { return None; }
        let type_ = self.parse_type().unwrap();
        if let Token::Operation { operator: Operator::Assign } = self.tokens.remove(0) {} else { return None; }

        if let Token::Semicolon = &self.tokens[0] {
            return Some(Statement::Let { identifier, type_, expression: None });
        }

        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::Semicolon = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Let { identifier, type_, expression: Some(expression) })
    }

    fn parse_type(&mut self) -> Option<ValueType> {
        return if let Some(Token::Type { type_ }) = self.get_type() {
            Some(
                match type_ {
                    TypeType::U64 => ValueType::U64,
                    TypeType::Char => ValueType::Char
                }
            )
        } else if let Token::OpenBracket = self.tokens.remove(0) {
            let type_ = self.parse_type().unwrap();
            if let Token::Comma = self.tokens.remove(0) {} else { return None; }
            let len = if let Some(Token::Literal { type_: LiteralType::Number, value }) = self.get_literal() { value } else { return None; };
            if let Token::ClosedBracket = self.tokens.remove(0) {} else { return None; }
            Some(
                ValueType::Array {
                    content_type: Box::new(type_),
                    len,
                }
            )
        } else { None };
    }

    fn parse_exit(&mut self) -> Option<Statement> {
        if let Token::OpenParent = self.tokens.remove(0) {} else { return None; }
        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::ClosedParent = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Exit { expression })
    }

    fn parse_print(&mut self) -> Option<Statement> {
        if let Token::OpenParent = self.tokens.remove(0) {} else { return None; }
        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::ClosedParent = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Print { expression })
    }

    fn parse_assign(&mut self, identifier: Literal) -> Option<Statement> {
        if let Token::Operation { operator: Operator::Assign } = self.tokens.remove(0) {} else { return None; }
        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::Semicolon = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Assign { identifier, expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expression = match self.tokens.remove(0) {
            Token::Literal { type_: LiteralType::Identifier, value } => Expression::IdentifierLiteral { value },
            Token::Literal { type_: LiteralType::Number, value } => Expression::NumberLiteral { value },
            Token::Literal { type_: LiteralType::Char, value } => Expression::NumberLiteral { value },
            Token::Literal { type_: LiteralType::String, value } => Expression::Array { content: Self::string_to_char_array(value) },
            Token::Operation { operator } => self.parse_prefix_expression(operator).unwrap(),
            Token::OpenParent => self.parse_grouped().unwrap(),
            Token::OpenBracket => self.parse_array().unwrap(),
            _ => return None
        };

        loop {
            if let Some(operator_precedence) = self.peek_precedence() {
                if !(precedence < operator_precedence) { break; }
                let operator = if let Some(Token::Operation { operator }) = self.get_operation() { operator } else { return None; };

                let infix = self.parse_infix_expression(left_expression, operator).unwrap();
                left_expression = infix;
            } else { break; }
        }

        Some(left_expression)
    }

    fn string_to_char_array(string: Literal) -> Vec<Expression> {
        let mut chars: Vec<Expression> = vec![];
        let mut iter = string.into_iter();
        while iter.len() != 0 {
            let c = iter.next().unwrap();
            if c == b'\\' {
                chars.push(Expression::CharLiteral { value: vec![b'\\', iter.next().unwrap()] });
            } else {
                chars.push(Expression::CharLiteral { value: vec![c] });
            }
        }
        chars
    }

    fn parse_grouped(&mut self) -> Option<Expression> {
        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::ClosedParent = self.tokens.remove(0) {} else { return None; };
        Some(expression)
    }

    fn parse_infix_expression(&mut self, left: Expression, operator: Operator) -> Option<Expression> {
        match operator {
            Operator::Plus |
            Operator::Minus |
            Operator::Times |
            Operator::Divide => {}
            _ => return None
        }
        let right = self.parse_expression(operator.get_precedence()).unwrap();
        Some(
            Expression::Operation {
                lhs: Box::new(left),
                operator,
                rhs:
                Box::new(right),
            }
        )
    }

    fn parse_prefix_expression(&mut self, operator: Operator) -> Option<Expression> {
        let right = self.parse_expression(Precedence::Prefix).unwrap();
        let expression = match operator {
            Operator::Times => Expression::Deref { value: Box::new(right) },
            _ => return None
        };

        Some(expression)
    }

    fn parse_array(&mut self) -> Option<Expression> {
        let mut content: Vec<Expression> = vec![];

        loop {
            let expression = self.parse_expression(Precedence::Lowest).unwrap();
            content.push(expression);

            if let Token::Comma = &self.tokens[0] {
                self.tokens.remove(0);
            } else {
                break;
            }
        }
        if let Token::ClosedBracket = self.tokens.remove(0) {} else { return None; }

        Some(Expression::Array { content })
    }

    fn get_keyword(&mut self) -> Option<Token> {
        match &self.tokens[0] {
            Token::Keyword { .. } => Some(self.tokens.remove(0)),
            _ => None
        }
    }

    fn get_literal(&mut self) -> Option<Token> {
        match &self.tokens[0] {
            Token::Literal { .. } => Some(self.tokens.remove(0)),
            _ => None
        }
    }

    fn get_type(&mut self) -> Option<Token> {
        match &self.tokens[0] {
            Token::Type { .. } => Some(self.tokens.remove(0)),
            _ => None
        }
    }

    fn get_operation(&mut self) -> Option<Token> {
        match &self.tokens[0] {
            Token::Operation { .. } => Some(self.tokens.remove(0)),
            _ => None
        }
    }

    fn peek_precedence(&mut self) -> Option<Precedence> {
        match &self.tokens[0] {
            Token::Operation { operator } => Some(operator.get_precedence()),
            _ => None
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Precedence {
    Lowest = 0,
    Sum = 1,
    Product = 2,
    Prefix = 3,
}

impl Operator {
    fn get_precedence(&self) -> Precedence {
        match self {
            Operator::Plus | Operator::Minus => Precedence::Sum,
            Operator::Times | Operator::Divide => Precedence::Product,
            Operator::And => Precedence::Prefix,
            _ => unreachable!()
        }
    }
}
