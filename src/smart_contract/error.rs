use std::fmt;
use crate::token::Token;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken,
    UnexpectedEOF,
    InvalidExpression,
    InvalidStatement,
    InvalidType,
    UndeclaredIdentifier(String),
    DuplicateIdentifier(String),
    TypeMismatch {
        expected: String,
        actual: String,
    },
    InvalidNumberOfArguments {
        expected: usize,
        actual: usize,
    },
    Custom(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken => write!(f, "Unexpected token"),
            ParserError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParserError::InvalidExpression => write!(f, "Invalid expression"),
            ParserError::InvalidStatement => write!(f, "Invalid statement"),
            ParserError::InvalidType => write!(f, "Invalid type"),
            ParserError::UndeclaredIdentifier(name) => write!(f, "Undeclared identifier: {}", name),
            ParserError::DuplicateIdentifier(name) => write!(f, "Duplicate identifier: {}", name),
            ParserError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch - expected: {}, actual: {}", expected, actual)
            }
            ParserError::InvalidNumberOfArguments { expected, actual } => {
                write!(
                    f,
                    "Invalid number of arguments - expected: {}, actual: {}",
                    expected, actual
                )
            }
            ParserError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

pub struct ErrorReporter {
    errors: Vec<(usize, usize, ParserError)>,
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter { errors: Vec::new() }
    }

    pub fn report(&mut self, line: usize, column: usize, error: ParserError) {
        self.errors.push((line, column, error));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn display_errors(&self) {
        for (line, column, error) in &self.errors {
            println!("[Line {}, Column {}] Error: {}", line, column, error);
        }
    }
}

pub trait ParserResult<T> {
    fn unwrap_or_report(self, reporter: &mut ErrorReporter, line: usize, column: usize) -> Option<T>;
}

impl<T, E: Into<ParserError>> ParserResult<T> for Result<T, E> {
    fn unwrap_or_report(self, reporter: &mut ErrorReporter, line: usize, column: usize) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                reporter.report(line, column, error.into());
                None
            }
        }
    }
}