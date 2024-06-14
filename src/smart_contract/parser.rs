use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value, BitwiseOperator, StringOperator};
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
        } else if line.starts_with("trigger_event") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(format!("Invalid trigger_event operation: {}", line));
            }
            let event_name = parts[1].to_string();
            let mut params = HashMap::new();
            for param in &parts[2..] {
                let kv: Vec<&str> = param.split('=').collect();
                if kv.len() != 2 {
                    return Err(format!("Invalid parameter in trigger_event operation: {}", param));
                }
                params.insert(kv[0].to_string(), Self::parse_value(kv[1])?);
            }
            Ok(Some(Operation::TriggerEvent { event_name, params }))
        } else if line.starts_with("external_call") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                return Err(format!("Invalid external_call operation: {}", line));
            }
            let contract_address = parts[1].to_string();
            let function_name = parts[2].to_string();
            let args = parts[3..].iter().map(|arg| Self::parse_expression(arg)).collect::<Result<Vec<_>, _>>()?;
            Ok(Some(Operation::ExternalCall { contract_address, function_name, args }))
            Err(format!("Invalid operation: '{}'. Please check the syntax and try again.", line))
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
        } else if expr.contains('&') {
            let parts: Vec<&str> = expr.split('&').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid bitwise AND expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BitwiseOp { left: Box::new(left), op: BitwiseOperator::And, right: Box::new(right) })
        } else if expr.contains('|') {
            let parts: Vec<&str> = expr.split('|').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid bitwise OR expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BitwiseOp { left: Box::new(left), op: BitwiseOperator::Or, right: Box::new(right) })
        } else if expr.contains('^') {
            let parts: Vec<&str> = expr.split('^').map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid bitwise XOR expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BitwiseOp { left: Box::new(left), op: BitwiseOperator::Xor, right: Box::new(right) })
        } else if expr.contains("<<") {
            let parts: Vec<&str> = expr.split("<<").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid bitwise shift left expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BitwiseOp { left: Box::new(left), op: BitwiseOperator::ShiftLeft, right: Box::new(right) })
        } else if expr.contains(">>") {
            let parts: Vec<&str> = expr.split(">>").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid bitwise shift right expression: {}", expr));
            }
            let left = Self::parse_expression(parts[0])?;
            let right = Self::parse_expression(parts[1])?;
            Ok(Expression::BitwiseOp { left: Box::new(left), op: BitwiseOperator::ShiftRight, right: Box::new(right) })
        } else if expr.contains("concat") {
            let parts: Vec<&str> = expr.split("concat").map(|part| part.trim()).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid string concatenation expression: {}", expr));
            }
            let args = parts.iter().map(|part| Self::parse_expression(part)).collect::<Result<Vec<_>, _>>()?;
            Ok(Expression::StringManipulation { op: StringOperator::Concat, args })
        } else if expr.contains("substring") {
            let parts: Vec<&str> = expr.split("substring").map(|part| part.trim()).collect();
            if parts.len() != 3 {
                return Err(format!("Invalid substring expression: {}", expr));
            }
            let args = parts.iter().map(|part| Self::parse_expression(part)).collect::<Result<Vec<_>, _>>()?;
            Ok(Expression::StringManipulation { op: StringOperator::Substring, args })
        } else if expr.contains("toupper") {
            let parts: Vec<&str> = expr.split("toupper").map(|part| part.trim()).collect();
            if parts.len() != 1 {
                return Err(format!("Invalid toupper expression: {}", expr));
            }
            let args = parts.iter().map(|part| Self::parse_expression(part)).collect::<Result<Vec<_>, _>>()?;
            Ok(Expression::StringManipulation { op: StringOperator::ToUpper, args })
        } else if expr.contains("tolower") {
            let parts: Vec<&str> = expr.split("tolower").map(|part| part.trim()).collect();
            if parts.len() != 1 {
                return Err(format!("Invalid tolower expression: {}", expr));
            }
            let args = parts.iter().map(|part| Self::parse_expression(part)).collect::<Result<Vec<_>, _>>()?;
            Ok(Expression::StringManipulation { op: StringOperator::ToLower, args })
        } else if expr.contains("replace") {
            let parts: Vec<&str> = expr.split("replace").map(|part| part.trim()).collect();
            if parts.len() != 3 {
                return Err(format!("Invalid replace expression: {}", expr));
            }
            let args = parts.iter().map(|part| Self::parse_expression(part)).collect::<Result<Vec<_>, _>>()?;
            Ok(Expression::StringManipulation { op: StringOperator::Replace, args })
            Ok(Expression::Variable(expr.to_string()))
        }
    }
}
