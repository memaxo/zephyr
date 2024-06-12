use crate::smart_contract::types::{Operation, Expression, BinaryOperator, UnaryOperator, Value};
use std::collections::HashSet;

pub struct Function {
    pub name: String,
    pub required_role: Role,
    pub arg_count: usize,
}

pub struct ExecutionContext {
    pub state: HashMap<String, Value>,
    pub transaction_stack: VecDeque<TransactionContext>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub roles: HashMap<String, HashSet<Role>>,
    pub functions: HashMap<String, Function>,
}

impl ExecutionContext {
    pub fn new(gas_limit: u64) -> Self {
        ExecutionContext {
            state: HashMap::new(),
            transaction_stack: VecDeque::new(),
            gas_used: 0,
            gas_limit,
            roles: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name.clone(), function);
    }

    pub fn check_function_permission(&self, user: &str, function_name: &str) -> Result<(), String> {
        if let Some(function) = self.functions.get(function_name) {
            self.check_permission(user, function.required_role.clone())
        } else {
            Err(format!("Function not found: {}", function_name))
        }
    }
}
use crate::smart_contract::gas::{calculate_operation_cost, calculate_expression_cost, GasCost};
use std::collections::HashMap;

pub struct Interpreter {
    pub gas_cost: GasCost,
}

impl Interpreter {
    pub fn new(gas_cost: GasCost) -> Self {
        Interpreter { gas_cost }
    }

    pub fn execute_operation(
        &self,
        operation: &Operation,
        context: &mut HashMap<String, Value>,
        gas_limit: &mut u64,
    ) -> Result<Option<Value>, String> {
        let gas_cost = calculate_operation_cost(operation, &self.gas_cost);
        if *gas_limit < gas_cost {
            return Err("Insufficient gas".to_string());
        }
        *gas_limit -= gas_cost;

        match operation {
            Operation::Set { key, value } => {
                let value = self.evaluate_expression(value, context, gas_limit)?;
                context.insert(key.clone(), value);
                Ok(None)
            },
            Operation::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition, context, gas_limit)?;
                if Self::truthy(&condition_value) {
                    Self::execute_operations(then_branch, context, gas_limit, &self.gas_cost)
                } else {
                    Self::execute_operations(else_branch, context, gas_limit, &self.gas_cost)
                }
            },
            Operation::Loop { condition, body } => {
                while Self::truthy(&self.evaluate_expression(condition, context, gas_limit)?) {
                    match Self::execute_operations(body, context, gas_limit, &self.gas_cost) {
                        Ok(_) => {},
                        Err(e) => {
                            if e == "break" {
                                break;
                            } else if e == "continue" {
                                continue;
                            } else {
                                return Err(e);
                            }
                        },
                    }
                }
                Ok(None)
            },
            Operation::FunctionCall { name, args } => {
                // Check function permissions
                context.check_function_permission(user, name)?;

                // Validate argument count
                if let Some(function) = context.functions.get(name) {
                    if args.len() != function.arg_count {
                        return Err(format!("Invalid argument count for function '{}'. Expected {}, got {}", name, function.arg_count, args.len()));
                    }
                } else {
                    return Err(format!("Function not found: {}", name));
                }

                // Evaluate arguments
                let evaluated_args: Result<Vec<Value>, String> = args.iter()
                    .map(|arg| self.evaluate_expression(arg, context, gas_limit))
                    .collect();

                let evaluated_args = evaluated_args?;

                // Execute function logic (to be implemented)
                // Update context with function return value if any
                unimplemented!("Function call logic not implemented")
            },
            Operation::Return { value } => {
                let return_value = self.evaluate_expression(value, context, gas_limit)?;
                Ok(Some(return_value))
            },
            Operation::Break => Err("break".to_string()),
            Operation::Continue => Err("continue".to_string()),
        }
    }

    fn execute_operations(
        operations: &[Operation],
        context: &mut HashMap<String, Value>,
        gas_limit: &mut u64,
        gas_cost: &GasCost,
    ) -> Result<Option<Value>, String> {
        for operation in operations {
            if let Some(value) = Interpreter::execute_operation(operation, context, gas_limit, gas_cost)? {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn evaluate_expression(
        &self,
        expression: &Expression,
        context: &HashMap<String, Value>,
        gas_limit: &mut u64,
    ) -> Result<Value, String> {
        let gas_cost = calculate_expression_cost(expression, &self.gas_cost);
        if *gas_limit < gas_cost {
            return Err("Insufficient gas".to_string());
        }
        *gas_limit -= gas_cost;

        match expression {
            Expression::Literal(value) => Ok(value.clone()),
            Expression::Variable(name) => context.get(name).cloned().ok_or_else(|| format!("Variable '{}' not found", name)),
            Expression::BinaryOp { left, op, right } => {
                let left_value = self.evaluate_expression(left, context, gas_limit)?;
                let right_value = self.evaluate_expression(right, context, gas_limit)?;
                Self::apply_binary_operator(&left_value, op, &right_value)
            },
            Expression::UnaryOp { op, expr } => {
                let value = self.evaluate_expression(expr, context, gas_limit)?;
                Self::apply_unary_operator(op, &value)
            },
            Expression::FunctionCall { .. } => unimplemented!("Function call not implemented"),
        }
    }

    fn apply_binary_operator(left: &Value, op: &BinaryOperator, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Integer(left), BinaryOperator::Add, Value::Integer(right)) => Ok(Value::Integer(left + right)),
            (Value::Integer(left), BinaryOperator::Subtract, Value::Integer(right)) => Ok(Value::Integer(left - right)),
            (Value::Integer(left), BinaryOperator::Multiply, Value::Integer(right)) => Ok(Value::Integer(left * right)),
            (Value::Integer(left), BinaryOperator::Divide, Value::Integer(right)) => {
                if *right == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Integer(left / right))
                }
            },
            (Value::Integer(left), BinaryOperator::Equals, Value::Integer(right)) => Ok(Value::Boolean(left == right)),
            (Value::Integer(left), BinaryOperator::NotEquals, Value::Integer(right)) => Ok(Value::Boolean(left != right)),
            (Value::Integer(left), BinaryOperator::GreaterThan, Value::Integer(right)) => Ok(Value::Boolean(left > right)),
            (Value::Integer(left), BinaryOperator::LessThan, Value::Integer(right)) => Ok(Value::Boolean(left < right)),
            (Value::Integer(left), BinaryOperator::GreaterThanOrEqual, Value::Integer(right)) => Ok(Value::Boolean(left >= right)),
            (Value::Integer(left), BinaryOperator::LessThanOrEqual, Value::Integer(right)) => Ok(Value::Boolean(left <= right)),
            (Value::Boolean(left), BinaryOperator::And, Value::Boolean(right)) => Ok(Value::Boolean(*left && *right)),
            (Value::Boolean(left), BinaryOperator::Or, Value::Boolean(right)) => Ok(Value::Boolean(*left || *right)),
            _ => Err(format!("Unsupported binary operation: {:?} {:?} {:?}", left, op, right)),
        }
    }

    fn apply_unary_operator(op: &UnaryOperator, value: &Value) -> Result<Value, String> {
        match (op, value) {
            (UnaryOperator::Negate, Value::Integer(value)) => Ok(Value::Integer(-value)),
            (UnaryOperator::Not, Value::Boolean(value)) => Ok(Value::Boolean(!value)),
            _ => Err(format!("Unsupported unary operation: {:?} {:?}", op, value)),
        }
    }

    fn truthy(value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Null => false,
        }
    }
}
