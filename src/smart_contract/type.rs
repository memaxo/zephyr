use crate::ast::*;
use crate::token::*;
use crate::error::*;

pub fn parse_type_annotation(parser: &mut Parser) -> Result<Type, ParserError> {
    match parser.current_token() {
        Some(Token::IntegerType) => {
            parser.consume_token();
            Ok(Type::Integer)
        }
        Some(Token::BooleanType) => {
            parser.consume_token();
            Ok(Type::Boolean)
        }
        Some(Token::StringType) => {
            parser.consume_token();
            Ok(Type::String)
        }
        Some(Token::Identifier(name)) => {
            parser.consume_token();
            Ok(Type::Struct(name))
        }
        Some(Token::LeftBracket) => parse_array_type(parser),
        Some(Token::LessThan) => parse_map_type(parser),
        _ => Err(ParserError::UnexpectedToken),
    }
}

fn parse_array_type(parser: &mut Parser) -> Result<Type, ParserError> {
    parser.expect_token(Token::LeftBracket)?;
    let element_type = parse_type_annotation(parser)?;
    parser.expect_token(Token::RightBracket)?;
    Ok(Type::Array(Box::new(element_type)))
}

fn parse_map_type(parser: &mut Parser) -> Result<Type, ParserError> {
    parser.expect_token(Token::LessThan)?;
    let key_type = parse_type_annotation(parser)?;
    parser.expect_token(Token::Comma)?;
    let value_type = parse_type_annotation(parser)?;
    parser.expect_token(Token::GreaterThan)?;
    Ok(Type::Map(Box::new(key_type), Box::new(value_type)))
}