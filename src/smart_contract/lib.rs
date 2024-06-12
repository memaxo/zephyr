pub mod types;
pub mod gas;
pub mod interpreter;
pub mod parser;
pub mod execution_context;
pub mod smart_contract_interface;

pub use types::{SmartContract, Operation, Value, Expression, BinaryOperator, UnaryOperator, TransactionContext};
pub use gas::{GasCost, calculate_operation_cost, calculate_expression_cost, calculate_contract_cost};
pub use interpreter::Interpreter;
pub use parser::Parser;
pub use execution_context::ExecutionContext;
pub use smart_contract_interface::SmartContractInterface;

use std::collections::HashMap;

pub fn execute_contract(contract: &SmartContract, gas_limit: u64) -> Result<HashMap<String, Value>, String> {
    let gas_cost = GasCost::default();
    let operations = Parser::parse_contract(&contract.code)?;

    let total_cost = calculate_contract_cost(&operations, &gas_cost);
    if total_cost > gas_limit {
        return Err(format!("Insufficient gas. Required: {}, Provided: {}", total_cost, gas_limit));
    }

    let mut context = ExecutionContext::new(gas_limit);
    let interpreter = Interpreter::new(gas_cost);

    for operation in &operations {
        interpreter.execute_operation(operation, &mut context.state, &mut context.gas_used)?;
    }

    Ok(context.state)
}
