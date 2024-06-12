use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value};
use log::info;

pub struct GasCost {
    pub set_cost: u64,
    pub get_cost: u64,
    pub op_cost: u64,
    pub func_call_cost: u64,
    pub loop_cost: u64,
    pub if_cost: u64,
    pub return_cost: u64,
    pub break_cost: u64,
    pub continue_cost: u64,
};
    info!("Calculated gas cost for operation {:?}: {}", operation, cost);
    cost

impl Default for GasCost {
    fn default() -> Self {
        GasCost {
            set_cost: 10,
            get_cost: 5,
            op_cost: 1,
            func_call_cost: 20,
            loop_cost: 5,
            if_cost: 5,
            return_cost: 5,
            break_cost: 5,
            continue_cost: 5,
        }
    }
}

pub fn calculate_operation_cost(operation: &Operation, gas_cost: &GasCost) -> u64 {
    let cost = match operation {
        Operation::Set { .. } => gas_cost.set_cost,
        Operation::If { then_branch, else_branch, .. } => {
            let then_cost: u64 = then_branch.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum();
            let else_cost: u64 = else_branch.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum();
            gas_cost.if_cost + then_cost + else_cost
        },
        Operation::Loop { body, .. } => {
            let body_cost: u64 = body.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum();
            gas_cost.loop_cost + body_cost
        },
        Operation::FunctionCall { args, .. } => {
            let args_cost: u64 = args.iter().map(|arg| calculate_expression_cost(arg, gas_cost)).sum();
            gas_cost.func_call_cost + args_cost
        },
        Operation::Return { .. } => gas_cost.return_cost,
        Operation::Break => gas_cost.break_cost,
        Operation::Continue => gas_cost.continue_cost,
    }
}

pub fn calculate_expression_cost(expression: &Expression, gas_cost: &GasCost) -> u64 {
    match expression {
        Expression::Literal(_) => gas_cost.op_cost,
        Expression::Variable(_) => gas_cost.get_cost,
        Expression::BinaryOp { left, right, .. } => {
            let left_cost = calculate_expression_cost(left, gas_cost);
            let right_cost = calculate_expression_cost(right, gas_cost);
            gas_cost.op_cost + left_cost + right_cost
        },
        Expression::UnaryOp { expr, .. } => {
            let expr_cost = calculate_expression_cost(expr, gas_cost);
            gas_cost.op_cost + expr_cost
        },
        Expression::FunctionCall { args, .. } => {
            let args_cost: u64 = args.iter().map(|arg| calculate_expression_cost(arg, gas_cost)).sum();
            gas_cost.func_call_cost + args_cost
        },
    }
}

pub fn calculate_contract_cost(operations: &[Operation], gas_cost: &GasCost) -> u64 {
    operations.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum()
}
