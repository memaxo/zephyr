use crate::ast::*;
use crate::token::*;
use crate::error::*;
use crate::expression::*;
use crate::statement::*;
use crate::type_annotation::parse_type_annotation;

pub fn parse_function_declaration(parser: &mut Parser) -> Result<Function, ParserError> {
    parser.expect_token(Token::Function)?;
    let name = parser.expect_identifier()?;

    parser.expect_token(Token::LeftParen)?;
    let parameters = parse_function_parameters(parser)?;
    parser.expect_token(Token::RightParen)?;

    let return_type = if parser.current_token() == Some(Token::Arrow) {
        parser.consume_token();
        Some(parse_type_annotation(parser)?)
    } else {
        None
    };

    let body = parse_block(parser)?;

    Ok(Function {
        name,
        parameters,
        return_type,
        body,
    })
}

fn parse_function_parameters(parser: &mut Parser) -> Result<Vec<(String, Type)>, ParserError> {
    let mut parameters = Vec::new();

    while parser.current_token() != Some(Token::RightParen) {
        let name = parser.expect_identifier()?;
        parser.expect_token(Token::Colon)?;
        let data_type = parse_type_annotation(parser)?;
        parameters.push((name, data_type));

        if parser.current_token() == Some(Token::Comma) {
            parser.consume_token();
        } else {
            break;
        }
    }

    Ok(parameters)
}

pub fn parse_function_call(parser: &mut Parser, name: String) -> Result<Expression, ParserError> {
    parser.expect_token(Token::LeftParen)?;
    let mut arguments = Vec::new();

    while parser.current_token() != Some(Token::RightParen) {
        let arg = parse_expression(parser)?;
        arguments.push(arg);

        if parser.current_token() == Some(Token::Comma) {
            parser.consume_token();
        } else {
            break;
        }
    }

    parser.expect_token(Token::RightParen)?;
    Ok(Expression::FunctionCall(name, arguments))
}