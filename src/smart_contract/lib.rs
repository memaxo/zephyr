mod ast;
mod token;
mod lexer;
mod parser;
mod expression;
mod statement;
mod function;
mod type_annotation;
mod contract;
mod symbol;
mod error;

pub use ast::*;
pub use token::*;
pub use lexer::*;
pub use parser::*;
pub use expression::*;
pub use statement::*;
pub use function::*;
pub use type_annotation::*;
pub use contract::*;
pub use symbol::*;
pub use error::*;

pub fn parse(input: &str) -> Result<Contract, Vec<ParserError>> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let mut error_reporter = ErrorReporter::new();

    let contract = parser.parse_contract(&mut error_reporter);

    if error_reporter.has_errors() {
        error_reporter.display_errors();
        Err(error_reporter.get_errors())
    } else {
        contract.map_err(|e| vec![e])
    }
}