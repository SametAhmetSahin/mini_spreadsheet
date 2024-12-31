use ast::{ASTCreator, Token, AST};
use ast_resolver::ASTResolver;
use core::panic;
use dependancy_graph::DependancyGraph;
use std::collections::HashMap;
use tokenizer::ExpressionTokenizer;

use super::{Cell, Expression, Index, ParsedCell};

pub mod ast;
pub mod dependancy_graph;
pub mod tokenizer;
pub mod ast_resolver;

pub struct CellParser {}

impl CellParser {
    pub fn parse_cell(cell: &mut Cell) {
        let raw_cell = &cell.raw_representation;
        if raw_cell.len() == 0 {
            unreachable!()
        }

        let parsed_cell = match raw_cell.chars().nth(0).expect("Should never fail") {
            '=' => Self::parse_expression(&raw_cell).unwrap(),
            num if num.is_digit(10) => match raw_cell.parse() {
                Ok(number) => ParsedCell::Number(number),
                Err(e) => panic!("Had error: -{e}- parsing number {raw_cell}"),
            },
            _ => ParsedCell::Text(raw_cell.to_string()),
        };

        cell.parsed_representation = Some(parsed_cell);
    }

    fn parse_expression(s: &str) -> Option<ParsedCell> {
        let tokens = ExpressionTokenizer::new(s[1..].chars().collect())
            .tokenize_expression()
            .ok()?;
        let dependencies = Self::find_dependants(&tokens);
        let ast = ASTCreator::new(tokens.into_iter()).parse().ok()?;
        let expr = Expression { ast, dependencies };
        Some(ParsedCell::Expr(expr))
    }

    fn find_dependants(tokens: &Vec<Token>) -> Vec<Index> {
        let cells = tokens
            .iter()
            .filter_map(|x| match x {
                Token::CellName(name) => Some(ASTResolver::get_cell_idx(name)),
                _ => None,
            })
            .collect();

        cells
    }

}
