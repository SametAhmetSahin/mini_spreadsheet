#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CellName(String),
    Number(f64),
    Plus,
    Minus,
    Division,
    Multiply,
    LParen,
    RParen,
}

impl Token {
    pub fn get_precedence(&self) -> usize {
        match &self {
            Token::Plus | Token::Minus => 1,
            Token::Division | Token::Multiply => 2,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    CellName(String),
    Value(Value),
    BinaryOp {
        op: Token,
        left: Box<AST>,
        right: Box<AST>,
    },
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub ast: AST,
    pub dependencies: Vec<Index>,
}

#[derive(Debug, Clone)]
pub enum ParsedCell {
    Value(Value),
    Expr(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Text(String),
    Number(f64),
}

impl Value {
    pub fn add(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Text(a), Value::Text(b)) => Some(Value::Text(a.clone() + &b)),
            _ => None,
        }
    }

    pub fn sub(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }

    pub fn div(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }

    pub fn mult(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError(pub String);

#[derive(Debug, Clone)]
pub enum ComputeError {
    ParseError(String),
    TypeError,
    UnfindableReference(String),
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub raw_representation: String,
    pub parsed_representation: Option<Result<ParsedCell, ParseError>>,
    pub computed_value: Option<Result<Value, ComputeError>>,
}

impl Cell {
    pub fn from_raw(raw: String) -> Self {
        Self {
            raw_representation: raw,
            parsed_representation: None,
            computed_value: None,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}
