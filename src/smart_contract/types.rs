use lazy_static::lazy_static;
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContract {
    pub name: String,
    pub code: String,
    pub version: String,
    pub author: String,
    pub gas_limit: u64,
}

impl SmartContract {
    pub fn validate(&self) -> bool {
        let name_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{0,31}$").unwrap();
        let version_regex = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();

        name_regex.is_match(&self.name)
            && version_regex.is_match(&self.version) 
            && self.gas_limit > 0
    }

    pub fn sanitize(&mut self) {
        self.name = self.name.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect();
        self.code = self.code.trim().to_string();
        self.author = self.author.trim().to_string();
    }
}

#[derive(Debug, Clone)]
lazy_static! {
    static ref ALLOWED_FUNCTIONS: HashSet<&'static str> = [
        "require",
        "revert",
        // Add more allowed functions
    ].iter().cloned().collect();
}

pub enum Operation {
    Set { key: String, value: Expression },
    If { condition: Expression, then_branch: Vec<Operation>, else_branch: Vec<Operation> },
    Loop { condition: Expression, body: Vec<Operation> },
    FunctionCall { name: String, args: Vec<Expression> },
    OracleCall { url: String, key: String },
    Return { value: Expression },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
impl Value {
    pub fn validate(&self) -> bool {
        match self {
            Value::String(s) => s.chars().all(|c| c.is_ascii()),
            Value::Array(arr) => arr.iter().all(|v| v.validate()),
            Value::Map(map) => map.values().all(|v| v.validate()),
            _ => true,
        }
    }
}

pub enum Value {
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
}

#[derive(Debug, Clone)]
impl Expression {
    pub fn validate(&self) -> bool {
        match self {
            Expression::Literal(value) => value.validate(),
            Expression::Variable(_) => true,
            Expression::BinaryOp { left, right, .. } => left.validate() && right.validate(),
            Expression::UnaryOp { expr, .. } => expr.validate(),
            Expression::FunctionCall { name, args } => {
                ALLOWED_FUNCTIONS.contains(name.as_str()) && args.iter().all(|arg| arg.validate())
            }
        }
    }
}

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
