pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    ArrayAccess(Box<Expression>, Box<Expression>),
    MapAccess(Box<Expression>, Box<Expression>),
    StructAccess(Box<Expression>, String),
}

pub enum Literal {
    Integer(i64),
    Boolean(bool),
    String(String),
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

pub enum UnaryOperator {
    Negate,
    Not,
}

pub enum Statement {
    VariableDeclaration(String, Option<Type>, Option<Expression>),
    Assignment(String, Expression),
    Expression(Expression),
    IfStatement(Expression, Vec<Statement>, Option<Vec<Statement>>),
    WhileStatement(Expression, Vec<Statement>),
    ReturnStatement(Option<Expression>),
    BreakStatement,
    ContinueStatement,
}

pub enum Type {
    Integer,
    Boolean,
    String,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Struct(String),
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

pub enum Operation {
    Statement(Statement),
    Function(Function),
    EventTrigger(String, Vec<(String, Expression)>),
    ExternalCall(Expression, String, Vec<Expression>),
}

pub struct Contract {
    pub name: String,
    pub state_variables: Vec<(String, Type)>,
    pub functions: Vec<Function>,
    pub operations: Vec<Operation>,
}