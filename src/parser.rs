use core::panic;
use ast::{ASTCreator, AST};
use tokenizer::ExpressionTokenizer;

use crate::raw_spreadsheet::{RawCell, RawSpreadSheet};
mod ast;
mod tokenizer;

#[derive(Debug)]
struct Expression {
    ast: AST,
}

#[derive(Debug)]
enum CellType {
    Text(String),
    Number(f64),
    Expr(Expression),
    Empty,
}

/// Represents a spread sheet where all cells have been tokenized
#[derive(Debug)]
pub struct ParsedSheet {
    rows: Vec<Vec<CellType>>,
    width: usize,
    height: usize,
}

pub struct CellParser {}

impl CellParser {
    pub fn parse_raw(ss: RawSpreadSheet) -> ParsedSheet {
        let parsed_rows = ss
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(Self::parse_cell).collect())
            .collect();

        ParsedSheet {
            rows: parsed_rows,
            width: ss.width,
            height: ss.height,
        }
    }

    fn parse_cell(rs: RawCell) -> CellType {
        let inner = rs.0;
        if inner.len() == 0 {
            return CellType::Empty;
        }

        match inner.chars().nth(0).expect("Should never fail") {
            '=' => Self::parse_expression(&inner).unwrap(),
            num if num.is_digit(10) => match inner.parse() {
                Ok(number) => CellType::Number(number),
                Err(e) => panic!("Had error: -{e}- parsing number {inner}"),
            },
            _ => CellType::Text(inner),
        }
    }

    fn parse_expression(s: &str) -> Option<CellType> {
        let tokens = ExpressionTokenizer::new(s[1..].chars().collect()).tokenize_expression().ok()?;
        let ast = ASTCreator::new(tokens.into_iter()).parse().ok()?;
        let expr = Expression { ast };
        Some(CellType::Expr(expr))
    }

}