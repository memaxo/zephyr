use crate::ast::*;
use crate::token::*;
use crate::error::*;

pub fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
    parse_binary_expression(parser, 0)
}

fn parse_binary_expression(parser: &mut Parser, precedence: u8) -> Result<Expression, ParserError> {
    let mut left = parse_unary_expression(parser)?;

    while let Some(op) = get_binary_operator(parser, precedence) {
        parser.consume_token();
        let right = parse_binary_expression(parser, op.precedence() + 1)?;
        left = Expression::BinaryOperation(op, Box::new(left), Box::new(right));
    }

    Ok(left)
}

fn parse_unary_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
    if let Some(op) = get_unary_operator(parser) {
        parser.consume_token();
        let expr = parse_unary_expression(parser)?;
        Ok(Expression::UnaryOperation(op, Box::new(expr)))
    } else {
        parse_primary_expression(parser)
    }
}

fn parse_primary_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
    match parser.current_token() {
        Some(Token::IntegerLiteral(value)) => {
            parser.consume_token();
            Ok(Expression::Literal(Literal::Integer(value)))
        }
        Some(Token::BooleanLiteral(value)) => {
            parser.consume_token();
            Ok(Expression::Literal(Literal::Boolean(value)))
        }
        Some(Token::StringLiteral(value)) => {
            parser.consume_token();
            Ok(Expression::Literal(Literal::String(value)))
        }
        Some(Token::Identifier(name)) => {
            parser.consume_token();
            if parser.current_token() == Some(Token::LeftParen) {
                parse_function_call(parser, name)
            } else {
                Ok(Expression::Variable(name))
            }
        }
        Some(Token::LeftParen) => {
            parser.consume_token();
            let expr = parse_expression(parser)?;
            parser.expect_token(Token::RightParen)?;
            Ok(expr)
        }
        _ => Err(ParserError::UnexpectedToken),
    }
}

fn parse_function_call(parser: &mut Parser, name: String) -> Result<Expression, ParserError> {
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

fn get_binary_operator(parser: &mut Parser, precedence: u8) -> Option<BinaryOperator> {
    match parser.current_token() {
        Some(Token::Plus) => Some(BinaryOperator::Add),
        Some(Token::Minus) => Some(BinaryOperator::Subtract),
        Some(Token::Asterisk) => Some(BinaryOperator::Multiply),
        Some(Token::Slash) => Some(BinaryOperator::Divide),
        Some(Token::Percent) => Some(BinaryOperator::Modulo),
        Some(Token::EqualsEquals) => Some(BinaryOperator::Equals),
        Some(Token::NotEquals) => Some(BinaryOperator::NotEquals),
        Some(Token::GreaterThan) => Some(BinaryOperator::GreaterThan),
        Some(Token::LessThan) => Some(BinaryOperator::LessThan),
        Some(Token::GreaterThanEquals) => Some(BinaryOperator::GreaterThanEquals),
        Some(Token::LessThanEquals) => Some(BinaryOperator::LessThanEquals),
        Some(Token::And) => Some(BinaryOperator::And),
        Some(Token::Or) => Some(BinaryOperator::Or),
        Some(Token::Ampersand) => Some(BinaryOperator::BitwiseAnd),
        Some(Token::Pipe) => Some(BinaryOperator::BitwiseOr),
        Some(Token::Caret) => Some(BinaryOperator::BitwiseXor),
        Some(Token::ShiftLeft) => Some(BinaryOperator::BitwiseShiftLeft),
        Some(Token::ShiftRight) => Some(BinaryOperator::BitwiseShiftRight),
        _ => None,
    }
}

fn get_unary_operator(parser: &mut Parser) -> Option<UnaryOperator> {
    match parser.current_token() {
        Some(Token::Minus) => Some(UnaryOperator::Negate),
        Some(Token::Bang) => Some(UnaryOperator::Not),
        _ => None,
    }
}