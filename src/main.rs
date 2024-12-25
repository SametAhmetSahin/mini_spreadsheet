use std::{cell, path::Path};

use raw_spreadsheet::{RawCell, RawSpreadSheet};
mod raw_spreadsheet;

#[derive(Debug)]
enum CloneCell {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
struct Expression {}

#[derive(Debug)]
enum Token {
    Text(String),
    Number(f64),
    Clone(CloneCell),
    Expr(Expression),
    Empty,
}

/// Represents a spread sheet where all cells have been tokenized
#[derive(Debug)]
struct ParsedSheet {
    rows: Vec<Vec<Token>>,
    width: usize,
    height: usize,
}

struct Parser {}

impl Parser {
    fn parse_raw(ss: RawSpreadSheet) -> ParsedSheet {
        let parsed_rows = ss
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(Self::parse_cell));

        ParsedSheet {
            rows: todo!(),
            width: ss.width,
            height: ss.height,
        }
    }

    fn parse_cell(rs: RawCell) -> Token {
        let inner = rs.0;
        if inner.len() == 0 {
            return Token::Empty;
        }

        match inner.chars().nth(0).unwrap() {
            '=' => Self::parse_expression(&inner).unwrap(),
            ':' => Self::parse_clone(&inner).unwrap(),
            _ => Token::Text(inner),
        }
    }

    fn parse_clone(s: &str) -> Option<Token> {
        todo!()
    }

    fn parse_expression(s: &str) -> Option<Token> {
        todo!()
    }
}

fn main() {
    let input = Path::new("csv").join("sum.csv");
    let raw_cells = RawSpreadSheet::new(input);
    println!("{}", &raw_cells);
    let parsed_cells =  Parser::parse_raw(raw_cells);
    println!("{:?}", parsed_cells);
}
