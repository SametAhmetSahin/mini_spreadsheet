use std::fmt::Display;

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
    Colon,
    Comma,
    FunctionName(String)
}

impl Token {
    #[must_use] pub fn get_precedence(&self) -> usize {
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
    Range {
        from: String,
        to: String,
    },
    FunctionCall {
        name: String,
        arguments: Vec<AST>, 
    }
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{s}"),
            Value::Number(num) => write!(f, "{num}"),
        }
    }
}

impl Value {
    #[must_use] pub fn add(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Text(a), Value::Text(b)) => Some(Value::Text(a.clone() + &b)),
            _ => None,
        }
    }

    #[must_use] pub fn sub(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }

    #[must_use] pub fn div(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }

    #[must_use] pub fn mult(&self, other: Value) -> Option<Value> {
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
    Cycle
}

impl Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeError::ParseError(_) => write!(f, "!PARSE ERROR!"),
            ComputeError::TypeError => write!(f, "!TYPE ERROR!"),
            ComputeError::UnfindableReference(_) => write!(f, "!REFERENCE ERROR!"),
            ComputeError::Cycle => write!(f, "!CYCLIC REFERENCE!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub needs_compute : bool,
    pub raw_representation: String,
    pub parsed_representation: Option<Result<ParsedCell, ParseError>>,
    pub computed_value: Option<Result<Value, ComputeError>>,
}

impl Cell {
    #[must_use]
    pub fn from_raw(raw: String) -> Self {
        Self {
            raw_representation: raw,
            parsed_representation: None,
            computed_value: None,
            needs_compute: true,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy, Ord, PartialOrd)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}
