use crate::ast::*;
use crate::token::*;
use crate::error::*;
use crate::expression::*;
use crate::statement::parse_statement;
use crate::function::parse_function_declaration;
use crate::type_annotation::parse_type_annotation;

pub enum Operation {
    // ... (Existing variants) ...
    CreateSubtask(String, String),
    AssignSubtask(String, String),
    SubmitSubtaskSolution(String, String),
}

pub struct Subtask {
    pub id: String,
    pub data: String,       // Could be a serialized representation of the subtask data
    pub dependencies: Vec<String>, // IDs of other subtasks this subtask depends on
    pub status: SubtaskStatus,
}

fn parse_subtask(parser: &mut Parser) -> Result<Subtask, ParserError> {
    parser.expect_identifier("subtask")?;
    let id = parser.expect_identifier()?;
    parser.expect_token(Token::LeftBrace)?;

    let mut data = String::new();
    let mut dependencies = Vec::new();
    let mut status = SubtaskStatus::Pending;

    while parser.current_token() != Some(Token::RightBrace) {
        match parser.current_token() {
            Some(Token::Identifier(name)) if name == "data" => {
                parser.expect_identifier("data")?;
                parser.expect_token(Token::Colon)?;
                data = parser.expect_string()?;
                parser.expect_token(Token::Semicolon)?;
            }
            Some(Token::Identifier(name)) if name == "depends_on" => {
                parser.expect_identifier("depends_on")?;
                parser.expect_token(Token::Colon)?;
                while parser.current_token() != Some(Token::Semicolon) {
                    let dependency = parser.expect_identifier()?;
                    dependencies.push(dependency);
                    if parser.current_token() == Some(Token::Comma) {
                        parser.consume_token();
                    } else {
                        break;
                    }
                }
                parser.expect_token(Token::Semicolon)?;
            }
            Some(Token::Identifier(name)) if name == "status" => {
                parser.expect_identifier("status")?;
                parser.expect_token(Token::Colon)?;
                status = match parser.expect_identifier()?.as_str() {
                    "Pending" => SubtaskStatus::Pending,
                    "Assigned" => SubtaskStatus::Assigned,
                    "Completed" => SubtaskStatus::Completed,
                    "Failed" => SubtaskStatus::Failed,
                    _ => return Err(ParserError::InvalidToken),
                };
                parser.expect_token(Token::Semicolon)?;
            }
            _ => return Err(ParserError::UnexpectedToken),
        }
    }

    parser.expect_token(Token::RightBrace)?;

    Ok(Subtask {
        id,
        data,
        dependencies,
        status,
    })
}

pub enum SubtaskStatus {
    Pending,
    Assigned,
    Completed,
    Failed,
}

pub fn parse_contract(parser: &mut Parser) -> Result<Contract, ParserError> {
    parser.expect_token(Token::Contract)?;
    let name = parser.expect_identifier()?;

    parser.expect_token(Token::LeftBrace)?;

    let mut state_variables = Vec::new();
    let mut functions = Vec::new();
    let mut subtasks = Vec::new();
    let mut operations = Vec::new();

    while parser.current_token() != Some(Token::RightBrace) {
        match parser.current_token() {
            Some(Token::Let) => {
                let state_variable = parse_state_variable(parser)?;
                state_variables.push(state_variable);
            }
            Some(Token::Function) => {
                let function = parse_function_declaration(parser)?;
                functions.push(function);
            }
            Some(Token::Identifier(name)) if name == "event" => {
                let event_trigger = parse_event_trigger(parser)?;
                operations.push(Operation::EventTrigger(event_trigger));
            }
            Some(Token::Identifier(name)) if name == "external" => {
                let external_call = parse_external_call(parser)?;
                operations.push(Operation::ExternalCall(external_call));
            }
            Some(Token::Identifier(name)) if name == "subtask" => {
                let subtask = parse_subtask(parser)?;
                subtasks.push(subtask);
            }
            _ => {
                let statement = parse_statement(parser)?;
                operations.push(Operation::Statement(statement));
            }
        }
    }

    parser.expect_token(Token::RightBrace)?;

    Ok(Contract {
        name,
        state_variables,
        functions,
        operations,
        subtasks,
    })
}

fn parse_state_variable(parser: &mut Parser) -> Result<(String, Type), ParserError> {
    parser.expect_token(Token::Let)?;
    let name = parser.expect_identifier()?;
    parser.expect_token(Token::Colon)?;
    let data_type = parse_type_annotation(parser)?;
    parser.expect_token(Token::Semicolon)?;
    Ok((name, data_type))
}

fn parse_event_trigger(parser: &mut Parser) -> Result<(String, Vec<(String, Expression)>), ParserError> {
    parser.expect_identifier("event")?;
    let event_name = parser.expect_identifier()?;
    parser.expect_token(Token::LeftParen)?;

    let mut parameters = Vec::new();
    while parser.current_token() != Some(Token::RightParen) {
        let name = parser.expect_identifier()?;
        parser.expect_token(Token::Colon)?;
        let value = parse_expression(parser)?;
        parameters.push((name, value));

        if parser.current_token() == Some(Token::Comma) {
            parser.consume_token();
        } else {
            break;
        }
    }

    parser.expect_token(Token::RightParen)?;
    parser.expect_token(Token::Semicolon)?;

    Ok((event_name, parameters))
}

fn parse_external_call(parser: &mut Parser) -> Result<(Expression, String, Vec<Expression>), ParserError> {
    parser.expect_identifier("external")?;
    let contract_address = parse_expression(parser)?;
    parser.expect_token(Token::Dot)?;
    let function_name = parser.expect_identifier()?;
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
    parser.expect_token(Token::Semicolon)?;

    Ok((contract_address, function_name, arguments))
}
