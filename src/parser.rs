use ast::{ASTCreator, AST};
use core::panic;
use dependancy_graph::DependancyGraph;
use std::collections::HashMap;
use tokenizer::ExpressionTokenizer;

use crate::raw_spreadsheet::{Index, RawCell, RawSpreadSheet};
mod ast;
mod dependancy_graph;
mod tokenizer;

#[derive(Debug)]
struct Expression {
    ast: AST,
}

#[derive(Debug)]
enum ParsedCell {
    Text(String),
    Number(f64),
    Expr(Expression),
}

/// Represents a spread sheet where all cells have been tokenized
#[derive(Debug)]
pub struct ParsedSheet {
    pub cells: HashMap<Index, ParsedCell>,
    pub dependencies: DependancyGraph,
    width: usize,
    height: usize,
}

pub struct CellParser {}

impl CellParser {
    pub fn parse_raw(ss: RawSpreadSheet) -> ParsedSheet {
        let parsed_rows = ss
            .cells
            .into_iter()
            .map(|row| (row.0, Self::parse_cell(row.1)))
            .collect();

        ParsedSheet {
            cells: parsed_rows,
            width: ss.width,
            dependencies: todo!(),
            height: ss.height,
        }
    }

    fn parse_cell(rs: RawCell) -> ParsedCell {
        let inner = rs.0;

        if inner.len() == 0 {
            unreachable!()
        }

        match inner.chars().nth(0).expect("Should never fail") {
            '=' => Self::parse_expression(&inner).unwrap(),
            num if num.is_digit(10) => match inner.parse() {
                Ok(number) => ParsedCell::Number(number),
                Err(e) => panic!("Had error: -{e}- parsing number {inner}"),
            },
            _ => ParsedCell::Text(inner),
        }
    }

    fn parse_expression(s: &str) -> Option<ParsedCell> {
        let tokens = ExpressionTokenizer::new(s[1..].chars().collect())
            .tokenize_expression()
            .ok()?;
        let ast = ASTCreator::new(tokens.into_iter()).parse().ok()?;
        let expr = Expression { ast };
        Some(ParsedCell::Expr(expr))
    }
}
