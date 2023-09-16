use crate::parser::expr::Expression;
use crate::parser::stmt::Statement;
use crate::tokenizer::token::{Keyword, Literal, LiteralType, Operator, Token};

pub mod expr;
pub mod stmt;

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
        if let Token::Operation { operator: Operator::Assign } = self.tokens.remove(0) {} else { return None; }

        if let Token::Semicolon = &self.tokens[0] {
            return Some(Statement::Let { identifier, expression: None });
        }

        let expression = self.parse_expression().unwrap();
        if let Token::Semicolon = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Let { identifier, expression: Some(expression) })
    }

    fn parse_exit(&mut self) -> Option<Statement> {
        todo!()
    }

    fn parse_print(&mut self) -> Option<Statement> {
        todo!()
    }

    fn parse_assign(&mut self, identifier: Literal) -> Option<Statement> {
        if let Token::Operation { operator: Operator::Assign } = self.tokens.remove(0) {} else { return None; }
        let expression = self.parse_expression().unwrap();
        if let Token::Semicolon = self.tokens.remove(0) {} else { return None; }

        Some(Statement::Assign { identifier, expression })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        todo!()
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
}
