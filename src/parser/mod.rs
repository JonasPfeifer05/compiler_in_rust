use std::iter::Peekable;
use std::vec::IntoIter;
use anyhow::bail;
use crate::parser::expr::Expression;
use crate::parser::r#type::ValueType;
use crate::parser::stmt::Statement;
use crate::tokenizer::token::{Keyword, Literal, literal_to_string, LiteralType, Operator, Token, TypeType};

pub mod expr;
pub mod stmt;
pub mod r#type;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Peekable<IntoIter<Token>>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse_statements(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while self.tokens.peek().is_some() {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    eprintln!("ERROR WHILE PARSING: {}", error);
                }
            }
        }

        statements
    }
    fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        if let Ok(Token::Keyword { keyword }) = self.get_keyword() {
            match keyword {
                Keyword::Let => self.parse_let(),
                Keyword::Exit => self.parse_exit(),
                Keyword::Print => self.parse_print(),
            }
        } else {
            self.parse_assign()
        }
    }

    fn parse_let(&mut self) -> anyhow::Result<Statement> {
        let identifier = if let Ok(Token::Literal { type_: LiteralType::Identifier, value }) = self.get_literal() {
            value
        } else {
            bail!("Let statement requires identifier to assign to!")
        };
        if let Token::Colon = self.consume_token()? {} else { bail!("Let statement requires ':' after assignee identifier!") }
        let type_ = self.parse_type()?;
        if let Token::Operation { operator: Operator::Assign } = self.consume_token()? {} else { bail!("Let statement requires '=' after the identifier declaration!") }

        if let Token::Semicolon = self.peek_token()? {
            return Ok(Statement::Let { identifier, type_, expression: None });
        }

        let expression = self.parse_expression(Precedence::Lowest)?;
        if let Token::Semicolon = self.consume_token()? {} else { bail!("Statement didnt end with ';'!") }

        Ok(Statement::Let { identifier, type_, expression: Some(expression) })
    }

    fn parse_type(&mut self) -> anyhow::Result<ValueType> {
        if let Token::Operation { operator: Operator::And } = self.peek_token()? {
            self.consume_token()?;
            return Ok(
                ValueType::Pointer {
                    points_to: Box::new(self.parse_type()?)
                }
            );
        }

        return if let Ok(Token::Type { type_ }) = self.get_type() {
            Ok(
                match type_ {
                    TypeType::U64 => ValueType::U64,
                    TypeType::U32 => ValueType::U32,
                    TypeType::U16 => ValueType::U16,
                    TypeType::U8 => ValueType::U8,
                    TypeType::Char => ValueType::Char,
                }
            )
        } else if let Token::OpenBracket = self.consume_token()? {
            let type_ = self.parse_type()?;
            if let Token::Comma = self.consume_token()? {} else { bail!("Array type expected comma after internal type descriptor!") }
            let len = if let Ok(Token::Literal { type_: LiteralType::Number, value }) = self.get_literal() { value } else { bail!("Expected number literal to describe the length of the array!") };
            if let Token::ClosedBracket = self.consume_token()? {} else { bail!("Array type didnt end with ']'!") }
            Ok(
                ValueType::Array {
                    content_type: Box::new(type_),
                    len: String::from_utf8_lossy(len.as_slice()).parse::<usize>().unwrap(),
                }
            )
        } else { bail!("Got unexpected token for a type!") };
    }

    fn parse_exit(&mut self) -> anyhow::Result<Statement> {
        if let Token::OpenParent = self.consume_token()? {} else { bail!("Expected '(' after function identifier!") }
        let expression = self.parse_expression(Precedence::Lowest).unwrap();
        if let Token::ClosedParent = self.consume_token()? {} else { bail!("Expected ')' after function parameters!") }

        Ok(Statement::Exit { expression })
    }

    fn parse_print(&mut self) -> anyhow::Result<Statement> {
        if let Token::OpenParent = self.consume_token()? {} else { bail!("Expected '(' after function identifier!") }
        let expression = self.parse_expression(Precedence::Lowest)?;
        if let Token::ClosedParent = self.consume_token()? {} else { bail!("Expected ')' after function parameters!") }

        Ok(Statement::Print { expression })
    }

    fn parse_assign(&mut self) -> anyhow::Result<Statement> {
        let assignee = self.parse_expression(Precedence::Lowest)?;

        if let Token::Operation { operator: Operator::Assign } = self.consume_token()? {} else { bail!("Expected '=' after the identifier in let statement!") }
        let expression = self.parse_expression(Precedence::Lowest)?;
        if let Token::Semicolon = self.consume_token()? {} else { bail!("Statement didnt end with ';'!") }

        Ok(Statement::Assign { assignee, expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> anyhow::Result<Expression> {
        let mut left_expression = match self.consume_token()? {
            Token::Literal { type_: LiteralType::Identifier, value } => Expression::IdentifierLiteral { value, type_: None },
            Token::Literal { type_: LiteralType::Number, value } => Self::parse_number_literal(value)?,
            Token::Literal { type_: LiteralType::Char, value } => Expression::CharLiteral { value },
            Token::Literal { type_: LiteralType::String, value } => Expression::Array { content: Self::string_to_char_array(value) },
            Token::Operation { operator } => self.parse_prefix_expression(operator)?,
            Token::OpenParent => self.parse_grouped()?,
            Token::OpenBracket => self.parse_array()?,
            _ => bail!("Invalid token found inside expression!")
        };

        while let Ok(operator_precedence) = self.peek_precedence() {
            if precedence >= operator_precedence { break; }

            let infix = if let Ok(Token::Operation { operator }) = self.get_operation() {
                self.parse_infix_expression(left_expression, operator)?
            } else if let Token::OpenBracket = self.consume_token()? {
                self.parse_access(left_expression)?
            } else { bail!("Invalid operator for infix operation found!") };

            left_expression = infix;
        }

        Ok(left_expression)
    }

    fn parse_number_literal(value: Literal) -> anyhow::Result<Expression> {
        let string_representation = literal_to_string(&value);
        let type_ = if string_representation.parse::<u8>().is_ok() {
            ValueType::U8
        } else if string_representation.parse::<u16>().is_ok() {
            ValueType::U16
        } else if string_representation.parse::<u32>().is_ok() {
            ValueType::U32
        } else if string_representation.parse::<u64>().is_ok() {
            ValueType::U64
        } else { bail!("To big integer literal found!"); };
        Ok(Expression::NumberLiteral { value, internal_type: type_ })
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

    fn parse_grouped(&mut self) -> anyhow::Result<Expression> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        if let Token::ClosedParent = self.consume_token()? {} else { bail!("Grouped expression didnt end with ')'!") };
        Ok(expression)
    }

    fn parse_infix_expression(&mut self, left: Expression, operator: Operator) -> anyhow::Result<Expression> {
        match operator {
            Operator::Plus |
            Operator::Minus |
            Operator::Times |
            Operator::Divide => {}
            _ => bail!("Found invalid infix operator!")
        }
        let right = self.parse_expression(operator.get_precedence()?)?;
        Ok(
            Expression::Operation {
                lhs: Box::new(left),
                operator,
                rhs:
                Box::new(right),
                type_: None,
            }
        )
    }

    fn parse_prefix_expression(&mut self, operator: Operator) -> anyhow::Result<Expression> {
        let right = self.parse_expression(Precedence::Prefix)?;

        let expression = match operator {
            Operator::Times => Expression::Deref { value: Box::new(right) },
            Operator::And => Expression::Reference { reference: Box::new(right) },
            _ => bail!("Found invalid prefix operator!")
        };

        Ok(expression)
    }

    fn parse_array(&mut self) -> anyhow::Result<Expression> {
        let mut content: Vec<Expression> = vec![];

        loop {
            let expression = self.parse_expression(Precedence::Lowest)?;
            content.push(expression);

            if let Token::Comma = self.peek_token()? {
                self.consume_token()?;
            } else {
                break;
            }
        }
        if let Token::ClosedBracket = self.consume_token()? {} else { bail!("Array expression didnt end with ']'!") }

        Ok(Expression::Array { content })
    }

    fn parse_access(&mut self, left: Expression) -> anyhow::Result<Expression> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if let Token::ClosedBracket = self.consume_token()? {} else { bail!("Access expression didnt end with ']'!") }
        Ok(Expression::Access { value: Box::new(left), index: Box::new(expression) })
    }

    fn get_keyword(&mut self) -> anyhow::Result<Token> {
        match self.tokens.peek() {
            Some(Token::Keyword { .. }) => Ok(self.consume_token()?),
            _ => bail!("There is no following keyword token!")
        }
    }

    fn get_literal(&mut self) -> anyhow::Result<Token> {
        match self.tokens.peek() {
            Some(Token::Literal { .. }) => Ok(self.consume_token()?),
            _ => bail!("There is no following literal token!")
        }
    }

    fn get_type(&mut self) -> anyhow::Result<Token> {
        match self.tokens.peek() {
            Some(Token::Type { .. }) => Ok(self.consume_token()?),
            _ => bail!("There is no following type token!")
        }
    }

    fn get_operation(&mut self) -> anyhow::Result<Token> {
        match self.tokens.peek() {
            Some(Token::Operation { .. }) => Ok(self.consume_token()?),
            _ => bail!("There is no following operator token!")
        }
    }

    fn consume_token(&mut self) -> anyhow::Result<Token> {
        if self.tokens.peek().is_none() { bail!("Tried to consume token but ran out of tokens!") }
        Ok(self.tokens.next().expect("THIS WILL NEVER OCCUR!"))
    }

    fn peek_token(&mut self) -> anyhow::Result<&Token> {
        match self.tokens.peek() {
            Some(token) => Ok(token),
            None => bail!("Tried to peek token but there are no more tokens!"),
        }
    }

    fn peek_precedence(&mut self) -> anyhow::Result<Precedence> {
        match self.tokens.peek() {
            Some(token) => Ok(token.get_precedence()?),
            None => bail!("Tried to peek precedence but there are no more tokens!"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Precedence {
    Lowest = 0,
    Sum = 1,
    Product = 2,
    Prefix = 3,
    Postfix = 4,
}

impl Token {
    fn get_precedence(&self) -> anyhow::Result<Precedence> {
        match self {
            Token::Operation {
                operator
            } => operator.get_precedence(),
            Token::OpenBracket => Ok(Precedence::Postfix),
            _ => bail!("Tried to get precedence of token that doesnt have a precedence!")
        }
    }
}

impl Operator {
    fn get_precedence(&self) -> anyhow::Result<Precedence> {
        match self {
            Operator::Plus |
            Operator::Minus => Ok(Precedence::Sum),
            Operator::Times |
            Operator::Divide => Ok(Precedence::Product),
            Operator::And => Ok(Precedence::Prefix),
            _ => bail!("Tried to get precedence of operation that doesnt have a precedence!")
        }
    }
}
