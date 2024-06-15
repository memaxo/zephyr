use crate::ast::*;
use crate::token::*;
use crate::error::*;
use crate::expression::*;
use crate::type_annotation::parse_type_annotation;

pub fn parse_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    match parser.current_token() {
        Some(Token::Let) => parse_variable_declaration(parser),
        Some(Token::Identifier(_)) => parse_assignment_or_expression(parser),
        Some(Token::If) => parse_if_statement(parser),
        Some(Token::While) => parse_while_statement(parser),
        Some(Token::Return) => parse_return_statement(parser),
        Some(Token::Break) => parse_break_statement(parser),
        Some(Token::Continue) => parse_continue_statement(parser),
        _ => Err(ParserError::UnexpectedToken),
    }
}

fn parse_variable_declaration(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::Let)?;
    let name = parser.expect_identifier()?;

    let data_type = if parser.current_token() == Some(Token::Colon) {
        parser.consume_token();
        Some(parse_type_annotation(parser)?)
    } else {
        None
    };

    let initializer = if parser.current_token() == Some(Token::Equals) {
        parser.consume_token();
        Some(parse_expression(parser)?)
    } else {
        None
    };

    parser.expect_token(Token::Semicolon)?;
    Ok(Statement::VariableDeclaration(name, data_type, initializer))
}

fn parse_assignment_or_expression(parser: &mut Parser) -> Result<Statement, ParserError> {
    let identifier = parser.expect_identifier()?;

    if parser.current_token() == Some(Token::Equals) {
        parser.consume_token();
        let value = parse_expression(parser)?;
        parser.expect_token(Token::Semicolon)?;
        Ok(Statement::Assignment(identifier, value))
    } else {
        let expr = parse_expression_from_identifier(parser, identifier)?;
        parser.expect_token(Token::Semicolon)?;
        Ok(Statement::Expression(expr))
    }
}

fn parse_if_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::If)?;
    parser.expect_token(Token::LeftParen)?;
    let condition = parse_expression(parser)?;
    parser.expect_token(Token::RightParen)?;

    let then_branch = parse_block(parser)?;

    let else_branch = if parser.current_token() == Some(Token::Else) {
        parser.consume_token();
        Some(parse_block(parser)?)
    } else {
        None
    };

    Ok(Statement::IfStatement(condition, then_branch, else_branch))
}

fn parse_while_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::While)?;
    parser.expect_token(Token::LeftParen)?;
    let condition = parse_expression(parser)?;
    parser.expect_token(Token::RightParen)?;

    let body = parse_block(parser)?;

    Ok(Statement::WhileStatement(condition, body))
}

fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::Return)?;

    let value = if parser.current_token() != Some(Token::Semicolon) {
        Some(parse_expression(parser)?)
    } else {
        None
    };

    parser.expect_token(Token::Semicolon)?;
    Ok(Statement::ReturnStatement(value))
}

fn parse_break_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::Break)?;
    parser.expect_token(Token::Semicolon)?;
    Ok(Statement::BreakStatement)
}

fn parse_continue_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parser.expect_token(Token::Continue)?;
    parser.expect_token(Token::Semicolon)?;
    Ok(Statement::ContinueStatement)
}

fn parse_block(parser: &mut Parser) -> Result<Vec<Statement>, ParserError> {
    parser.expect_token(Token::LeftBrace)?;

    let mut statements = Vec::new();
    while parser.current_token() != Some(Token::RightBrace) {
        let stmt = parse_statement(parser)?;
        statements.push(stmt);
    }

    parser.expect_token(Token::RightBrace)?;
    Ok(statements)
}

fn parse_expression_from_identifier(parser: &mut Parser, identifier: String) -> Result<Expression, ParserError> {
    // Implement parsing of expressions starting with an identifier
    // This includes function calls, array indexing, struct field access, etc.
    // You can use the existing `parse_expression` function as a starting point
    // and modify it to handle these specific cases.
    unimplemented!()
}