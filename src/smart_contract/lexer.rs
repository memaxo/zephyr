use std::str::Chars;

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input: chars,
            current_char,
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.column += 1;
        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn is_eof(&self) -> bool {
        self.current_char.is_none()
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.current_char, Some(c) if c.is_whitespace())
    }

    fn is_digit(&self) -> bool {
        matches!(self.current_char, Some(c) if c.is_digit(10))
    }

    fn is_alpha(&self) -> bool {
        matches!(self.current_char, Some(c) if c.is_alphabetic())
    }

    fn is_alphanumeric(&self) -> bool {
        matches!(self.current_char, Some(c) if c.is_alphanumeric())
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.is_whitespace() {
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while !self.is_eof() && self.is_alphanumeric() {
            identifier.push(self.current_char.unwrap());
            self.advance();
        }
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while !self.is_eof() && self.is_digit() {
            number.push(self.current_char.unwrap());
            self.advance();
        }
        number
    }

    fn read_string(&mut self) -> Result<String, LexerError> {
        let mut string = String::new();
        self.advance(); // Consume the opening quote
        while !self.is_eof() && self.current_char != Some('"') {
            if self.current_char == Some('\\') {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('r') => string.push('\r'),
                    Some('t') => string.push('\t'),
                    Some('"') => string.push('"'),
                    Some('\\') => string.push('\\'),
                    _ => return Err(LexerError::InvalidEscapeSequence),
                }
            } else {
                string.push(self.current_char.unwrap());
            }
            self.advance();
        }
        if self.is_eof() {
            Err(LexerError::UnterminatedString)
        } else {
            self.advance(); // Consume the closing quote
            Ok(string)
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        if self.is_eof() {
            return Ok(Token::EOF);
        }
        match self.current_char {
            Some(c) if self.is_alpha() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "if" => Ok(Token::If),
                    "else" => Ok(Token::Else),
                    "while" => Ok(Token::While),
                    "fn" => Ok(Token::Function),
                    "true" => Ok(Token::BooleanLiteral(true)),
                    "false" => Ok(Token::BooleanLiteral(false)),
                    _ => Ok(Token::Identifier(identifier)),
                }
            }
            Some(c) if self.is_digit() => {
                let number = self.read_number();
                match number.parse::<i64>() {
                    Ok(value) => Ok(Token::IntegerLiteral(value)),
                    Err(_) => Err(LexerError::InvalidNumber),
                }
            }
            Some('"') => {
                let string = self.read_string()?;
                Ok(Token::StringLiteral(string))
            }
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(Token::EqualsEquals)
                } else {
                    Ok(Token::Equals)
                }
            }
            Some('+') => Ok(Token::Plus),
            Some('-') => Ok(Token::Minus),
            Some('*') => Ok(Token::Asterisk),
            Some('/') => Ok(Token::Slash),
            Some('%') => Ok(Token::Percent),
            Some('&') => {
                if self.peek() == Some('&') {
                    self.advance();
                    Ok(Token::And)
                } else {
                    Ok(Token::Ampersand)
                }
            }
            Some('|') => {
                if self.peek() == Some('|') {
                    self.advance();
                    Ok(Token::Or)
                } else {
                    Ok(Token::Pipe)
                }
            }
            Some('!') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(Token::NotEquals)
                } else {
                    Ok(Token::Bang)
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(Token::GreaterThanEquals)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(Token::LessThanEquals)
                } else {
                    Ok(Token::LessThan)
                }
            }
            Some('(') => Ok(Token::LeftParen),
            Some(')') => Ok(Token::RightParen),
            Some('[') => Ok(Token::LeftBracket),
            Some(']') => Ok(Token::RightBracket),
            Some('{') => Ok(Token::LeftBrace),
            Some('}') => Ok(Token::RightBrace),
            Some(',') => Ok(Token::Comma),
            Some(':') => Ok(Token::Colon),
            Some('.') => Ok(Token::Dot),
            Some(';') => Ok(Token::Semicolon),
            _ => Err(LexerError::UnexpectedCharacter),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    // Keywords
    If,
    Else,
    While,
    Function,

    // Literals
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    StringLiteral(String),

    // Identifiers
    Identifier(String),

    // Operators
    Equals,
    EqualsEquals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    And,
    Or,
    Bang,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    NotEquals,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Dot,
    Semicolon,

    // Special tokens
    EOF,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter,
    InvalidNumber,
    UnterminatedString,
    InvalidEscapeSequence,
}