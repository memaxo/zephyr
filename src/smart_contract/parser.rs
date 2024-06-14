use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value, BitwiseOperator, StringOperator};
use log::info;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse_contract(&mut self) -> Result<Vec<Operation>, String> {
        let mut operations = Vec::new();
        while !self.is_at_end() {
            let operation = self.parse_operation()?;
            operations.push(operation);
        }
        Ok(operations)
    }

    fn parse_operation(&mut self) -> Result<Operation, String> {
        match self.current_token() {
            Token::Set => self.parse_set_operation(),
            Token::If => self.parse_if_operation(),
            Token::Loop => self.parse_loop_operation(),
            Token::Break => self.parse_break_operation(),
            Token::Continue => self.parse_continue_operation(),
            Token::Return => self.parse_return_operation(),
            Token::TriggerEvent => self.parse_trigger_event_operation(),
            Token::ExternalCall => self.parse_external_call_operation(),
            _ => Err("Invalid operation".to_string()),
        }
    }

    fn parse_set_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::Set)?;
        let key = self.parse_identifier()?;
        self.expect_token(Token::Equals)?;
        let value = self.parse_expression(0)?;
        Ok(Operation::Set { key, value })
    }

    fn parse_if_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::If)?;
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RightParen)?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.current_token() == &Token::Else {
            self.consume_token();
            self.parse_block()?
        } else {
            Vec::new()
        };
        Ok(Operation::If { condition, then_branch, else_branch })
    }
    
    fn parse_loop_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::Loop)?;
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RightParen)?;
        let body = self.parse_block()?;
        Ok(Operation::Loop { condition, body })
    }
    
    fn parse_break_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::Break)?;
        Ok(Operation::Break)
    }
    
    fn parse_continue_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::Continue)?;
        Ok(Operation::Continue)
    }
    
    fn parse_return_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::Return)?;
        let value = self.parse_expression(0)?;
        Ok(Operation::Return { value })
    }
    
    fn parse_trigger_event_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::TriggerEvent)?;
        let event_name = self.parse_identifier()?;
        let mut params = HashMap::new();
        while self.current_token() != &Token::Semicolon {
            let key = self.parse_identifier()?;
            self.expect_token(Token::Equals)?;
            let value = self.parse_expression(0)?;
            params.insert(key, value);
        }
        Ok(Operation::TriggerEvent { event_name, params })
    }
    
    fn parse_external_call_operation(&mut self) -> Result<Operation, String> {
        self.expect_token(Token::ExternalCall)?;
        let contract_address = self.parse_expression(0)?;
        self.expect_token(Token::Dot)?;
        let function_name = self.parse_identifier()?;
        self.expect_token(Token::LeftParen)?;
        let mut args = Vec::new();
        while self.current_token() != &Token::RightParen {
            let arg = self.parse_expression(0)?;
            args.push(arg);
            if self.current_token() == &Token::Comma {
                self.consume_token();
            }
        }
        self.expect_token(Token::RightParen)?;
        Ok(Operation::ExternalCall { contract_address, function_name, args })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Operation>, String> {
        let mut operations = Vec::new();
        self.expect_token(Token::LeftBrace)?;
        while self.current_token() != &Token::RightBrace {
            let operation = self.parse_operation()?;
            operations.push(operation);
        }
        self.expect_token(Token::RightBrace)?;
        Ok(operations)
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.parse_operator() {
            if op.precedence() < precedence {
                break;
            }
            self.consume_token();
            let right = self.parse_expression(op.precedence() + 1)?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::IntegerLiteral(value) => {
                self.consume_token();
                Ok(Expression::Literal(Value::Integer(value)))
            }
            Token::BooleanLiteral(value) => {
                self.consume_token();
                Ok(Expression::Literal(Value::Boolean(value)))
            }
            Token::StringLiteral(value) => {
                self.consume_token();
                Ok(Expression::Literal(Value::String(value)))
            }
            Token::Identifier(name) => {
                self.consume_token();
                Ok(Expression::Variable(name))
            }
            Token::LeftParen => {
                self.consume_token();
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            _ => Err("Invalid primary expression".to_string()),
        }
    }

    fn parse_operator(&mut self) -> Option<BinaryOperator> {
        match self.current_token() {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Subtract),
            Token::Asterisk => Some(BinaryOperator::Multiply),
            Token::Slash => Some(BinaryOperator::Divide),
            Token::EqualsEquals => Some(BinaryOperator::Equals),
            Token::NotEquals => Some(BinaryOperator::NotEquals),
            Token::GreaterThan => Some(BinaryOperator::GreaterThan),
            Token::LessThan => Some(BinaryOperator::LessThan),
            Token::GreaterThanEquals => Some(BinaryOperator::GreaterThanOrEqual),
            Token::LessThanEquals => Some(BinaryOperator::LessThanOrEqual),
            Token::And => Some(BinaryOperator::And),
            Token::Or => Some(BinaryOperator::Or),
            _ => None,
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume_token(&mut self) {
        self.current += 1;
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token() == &expected {
            self.consume_token();
            Ok(())
        } else {
            Err(format!("Expected token {:?}, found {:?}", expected, self.current_token()))
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.current_token() {
            Token::Identifier(name) => {
                self.consume_token();
                Ok(name.clone())
            }
            _ => Err("Expected identifier".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Set,
    If,
    Loop,
    Break,
    Continue,
    Return,
    TriggerEvent,
    ExternalCall,
    Identifier(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    And,
    Or,
    Equals,
    LeftParen,
    RightParen,
}

impl Token {
    fn precedence(&self) -> u8 {
        match self {
            Token::Or => 1,
            Token::And => 2,
            Token::EqualsEquals | Token::NotEquals => 3,
            Token::GreaterThan | Token::LessThan | Token::GreaterThanEquals | Token::LessThanEquals => 4,
            Token::Plus | Token::Minus => 5,
            Token::Asterisk | Token::Slash => 6,
            _ => 0,
        }
    }
}