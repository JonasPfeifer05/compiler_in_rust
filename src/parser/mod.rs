use crate::parser::stmt::Statement;
use crate::tokenizer::token::{LiteralType, Token};

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
            todo!()
        } else if let Token::Literal {
            type_: LiteralType::Identifier,
            value
        } = &self.tokens[0] {
            todo!()
        } else {
            None
        }
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
