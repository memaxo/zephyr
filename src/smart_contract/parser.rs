use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value};
use log::info;
use std::collections::HashMap;

pub struct Parser;

impl Parser {
    pub fn parse_contract(code: &str) -> Result<Vec<Operation>, String> {
        let mut operations = Vec::new();
        let lines: Vec<&str> = code.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if let Some(operation) = Self::parse_operation(line)? {
                operations.push(operation);
            }
            i += 1;
        }

        info!("Parsed contract with {} operations", operations.len());
        Ok(operations)
    }

    fn parse_operation(line: &str) -> Result<Option<Operation>, String> {
        if line.starts_with("set") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 4 || parts[2] != "=" {
                return Err(format!("Invalid set operation: {}", line));
            }
            let key = parts[1].trim().to_string();
            let value = Self::parse_expression(parts[3])?;
            Ok(Some(Operation::Set { key, value }))
        } else if line.starts_with("if") {
            let condition = Self::parse_expression(&line[3..line.len() - 1])?;
            Ok(Some(Operation::If { condition, then_branch: Vec::new(), else_branch: Vec::new() }))
        } else if line.starts_with("else") {
            Ok(None)
        } else if line.starts_with("endif") {
            Ok(None)
        } else if line.starts_with("while") {
            let condition = Self::parse_expression(&line[6..line.len() - 1])?;
            Ok(Some(Operation::Loop { condition, body: Vec::new() }))
        } else if line.starts_with("endwhile") {
            Ok(None)
        } else if line.starts_with("break") {
            Ok(Some(Operation::Break))
        } else if line.starts_with("continue") {
            Ok(Some(Operation::Continue))
        } else if line.starts_with("return") {
            let value = Self::parse_expression(&line[7..])?;
            Ok(Some(Operation::Return { value }))
        } else {
            Err(format!("Invalid operation: {}", line))
        }
    }

    fn parse_expression(expr: &str) -> Result<Expression, String> {
        let expr = expr.trim();
        if let Ok(value) = expr.parse::<i64>() {
            Ok(Expression::Literal(Value::Integer(value)))
        } else if let Ok(value) = expr.parse::<bool>() {
            Ok(Expression::Literal(Value::Boolean(value)))
        } else if expr.starts_with('"') && expr.ends_with('"') {
            let value = expr[1..expr.len() - 1].to_string();
            Ok(Expression::Literal(Value::String(value)))
        } else if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Add, right: Box::new(right) })
        } else if expr.contains('-') {
            let parts: Vec<&str> = expr.split('-').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Subtract, right: Box::new(right) })
        } else if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Multiply, right: Box::new(right) })
        } else if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Divide, right: Box::new(right) })
        } else if expr.contains("==") {
            let parts: Vec<&str> = expr.split("==").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Equals, right: Box::new(right) })
        } else if expr.contains("!=") {
            let parts: Vec<&str> = expr.split("!=").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::NotEquals, right: Box::new(right) })
        } else if expr.contains('>') {
            let parts: Vec<&str> = expr.split('>').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::GreaterThan, right: Box::new(right) })
        } else if expr.contains('<') {
            let parts: Vec<&str> = expr.split('<').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::LessThan, right: Box::new(right) })
        } else if expr.contains(">=") {
            let parts: Vec<&str> = expr.split(">=").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::GreaterThanOrEqual, right: Box::new(right) })
        } else if expr.contains("<=") {
            let parts: Vec<&str> = expr.split("<=").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::LessThanOrEqual, right: Box::new(right) })
        } else if expr.contains("&&") {
            let parts: Vec<&str> = expr.split("&&").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::And, right: Box::new(right) })
        } else if expr.contains("||") {
            let parts: Vec<&str> = expr.split("||").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid binary expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BinaryOp { left: Box::new(left), op: BinaryOperator::Or, right: Box::new(right) })
        } else if expr.starts_with('-') {
            let value = Self::parse_expression(&expr[1..])?;
            Ok(Expression::UnaryOp { op: UnaryOperator::Negate, expr: Box::new(value) })
        } else if expr.starts_with('!') {
            let value = Self::parse_expression(&expr[1..])?;
            Ok(Expression::UnaryOp { op: UnaryOperator::Not, expr: Box::new(value) })
        } else {
            Ok(Expression::Variable(expr.to_string()))
        }
    }
}
