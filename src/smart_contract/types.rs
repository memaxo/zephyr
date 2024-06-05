use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContract {
    pub name: String,
    pub code: String,
    pub version: String,
    pub author: String,
    pub gas_limit: u64,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Set { key: String, value: Expression },
    If { condition: Expression, then_branch: Vec<Operation>, else_branch: Vec<Operation> },
    Loop { condition: Expression, body: Vec<Operation> },
    FunctionCall { name: String, args: Vec<Expression> },
    Return { value: Expression },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Value),
    Variable(String),
    BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    UnaryOp { op: UnaryOperator, expr: Box<Expression> },
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub changes: HashMap<String, Value>,
}