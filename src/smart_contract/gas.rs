use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value};

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
}

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
    match operation {
        Operation::Set { .. } => gas_cost.set_cost,
        Operation::If { then_branch, else_branch, .. } => {
            gas_cost.if_cost
                + then_branch.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum::<u64>()
                + else_branch.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum::<u64>()
        },
        Operation::Loop { body, .. } => {
            gas_cost.loop_cost + body.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum::<u64>()
        },
        Operation::FunctionCall { args, .. } => {
            gas_cost.func_call_cost + args.iter().map(|arg| calculate_expression_cost(arg, gas_cost)).sum::<u64>()
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
            gas_cost.op_cost
                + calculate_expression_cost(left, gas_cost)
                + calculate_expression_cost(right, gas_cost)
        },
        Expression::UnaryOp { expr, .. } => gas_cost.op_cost + calculate_expression_cost(expr, gas_cost),
        Expression::FunctionCall { args, .. } => {
            gas_cost.func_call_cost + args.iter().map(|arg| calculate_expression_cost(arg, gas_cost)).sum::<u64>()
        },
    }
}

pub fn calculate_contract_cost(operations: &[Operation], gas_cost: &GasCost) -> u64 {
    operations.iter().map(|op| calculate_operation_cost(op, gas_cost)).sum()
}