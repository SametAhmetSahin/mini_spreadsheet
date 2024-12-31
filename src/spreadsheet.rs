use parser::{ast::AST, ast_resolver::ASTResolver, dependancy_graph::DependancyGraph, CellParser};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
mod parser;

#[derive(Debug)]
pub struct Expression {
    ast: AST,
    dependencies: Vec<Index>,
}

#[derive(Debug)]
pub enum ParsedCell {
    Text(String),
    Number(f64),
    Expr(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Text(String),
    Number(f64),
}

impl Value {
    fn add(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Text(a), Value::Text(b)) => Some(Value::Text(a.clone() + &b)),
            _ => None,
        }
    }

    fn sub(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }

    fn div(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }

    fn mult(&self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }
}



#[derive(Debug)]
pub struct Cell {
    pub raw_representation: String,
    pub parsed_representation: Option<ParsedCell>,
    pub computed_value: Option<Value>,
}

impl Cell {
    fn from_raw(raw: String) -> Self {
        Self {
            raw_representation: raw,
            parsed_representation: None,
            computed_value: None,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
pub struct Index {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Default)]
pub struct SpreadSheet {
    cells: HashMap<Index, Cell>,
    dependencies: DependancyGraph,
}

impl SpreadSheet {
    pub fn add_and_parse_raw(&mut self, index: Index, mut cell: Cell) {
        CellParser::parse_cell(&mut cell);
        if let Some(ParsedCell::Expr(Expression {
            ref dependencies, ..
        })) = cell.parsed_representation
        {
            self.dependencies.add_cell(index, dependencies);
        }
        self.cells.insert(index, cell);
    }

    pub fn compute_cell(&mut self, cell: Cell) {
        todo!()
    }

    pub fn from_file_path(input_path: PathBuf) -> Self {
        let mut buffer = String::new();
        let mut f = File::open(input_path).expect("Cannot open file");
        f.read_to_string(&mut buffer)
            .expect("Cannot read file to string");

        let mut spreadsheet = Self::default();

        for (y, line) in buffer.lines().enumerate() {
            for (x, cell) in line.split('|').enumerate() {
                let cell = cell.trim().to_string();
                if cell.is_empty() {
                    continue;
                }
                spreadsheet.add_and_parse_raw(Index { x, y }, Cell::from_raw(cell));
            }
        }

        spreadsheet
    }
}
