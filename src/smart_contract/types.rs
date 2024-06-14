use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use serde::{Serialize, Deserialize};
use pqcrypto_dilithium::dilithium2::{self, PublicKey, SecretKey, sign, verify};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartContract {
    pub name: String,
    pub code: String,
    pub version: String,
    pub author: String,
    pub gas_limit: u64,
    pub signature: Vec<u8>,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IBCPacketData {
    pub sequence: u64,
    pub timeout_height: u64,
    pub timeout_timestamp: u64,
    pub source_port: String,
    pub source_channel: String,
    pub dest_port: String,
    pub dest_channel: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrossChainMessage {
    pub source_chain: String,
    pub destination_chain: String,
    pub packet_data: IBCPacketData,
    pub timestamp: u64,
    pub signatures: Vec<String>,
    pub public_key: PublicKey,
    pub signature: Vec<u8>,
}

impl CrossChainMessage {
    pub fn verify_signature(&self) -> bool {
        verify(&self.signature, &serde_json::to_vec(self).unwrap(), &self.public_key).is_ok()
    }
}

pub enum CrossChainOperation {
    SendMessage { message: CrossChainMessage },
    ReceiveMessage { message: CrossChainMessage },
    QueryState { chain_id: String, key: String },
    TransferAssets { chain_id: String, amount: u64 },
    HTLCLock { htlc: AtomicSwap },
    HTLCUnlock { htlc: AtomicSwap, secret: Vec<u8> },
    HTLCRefund { htlc: AtomicSwap },
    IBCReceivePacket { packet: IBCPacketData },
    IBCAcknowledgePacket { packet: IBCPacketData },
    IBCTimeoutPacket { packet: IBCPacketData },
    OracleRequest { request: OracleRequest },
    OracleResponse { request_id: u64 },
    OracleQuery { query: String },
}

impl CrossChainOperation {
    pub fn verify(&self) -> bool {
        match self {
            CrossChainOperation::SendMessage { message } => message.verify_signature(),
            CrossChainOperation::ReceiveMessage { message } => message.verify_signature(),
            _ => true,
        }
    }
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
        info!("Sanitized contract name: {}", self.name);
        self.code = self.code.trim().to_string();
        self.author = self.author.trim().to_string();
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StringOperator {
    Concat,
    Substring,
    ToUpper,
    ToLower,
    Replace,
}
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone)]
pub enum StringOperator {
    Concat,
    Substring,
    ToUpper,
    ToLower,
    Replace,
}
lazy_static! {
    static ref ALLOWED_FUNCTIONS: HashSet<&'static str> = [
        "require",
        "revert",
        // Add more allowed functions
    ].iter().cloned().collect();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    Set { key: String, value: Expression },
    If { condition: Expression, then_branch: Vec<Operation>, else_branch: Vec<Operation> },
    Loop { condition: Expression, body: Vec<Operation> },
    FunctionCall { name: String, args: Vec<Expression> },
    OracleCall { url: String, key: String },
    Return { value: Expression },
    CrossChain(CrossChainOperation),
    Break,
    Continue,
    TriggerEvent { event_name: String, params: HashMap<String, Value> },
    ExternalCall { contract_address: String, function_name: String, args: Vec<Expression> },
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
    FixedPoint(f64),
    CustomStruct(HashMap<String, Value>),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expression {
    Literal(Value),
    Variable(String),
    BinaryOp { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    UnaryOp { op: UnaryOperator, expr: Box<Expression> },
    FunctionCall { name: String, args: Vec<Expression> },
    BitwiseOp { left: Box<Expression>, op: BitwiseOperator, right: Box<Expression> },
    StringManipulation { op: StringOperator, args: Vec<Expression> },
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitwiseNot,
}

pub trait CrossChainToken {
    fn transfer(&self, recipient: String, amount: u64) -> Result<(), String>;
    fn balance_of(&self, owner: String) -> Result<u64, String>;
    fn approve(&self, spender: String, amount: u64) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub changes: HashMap<String, Value>,
}
