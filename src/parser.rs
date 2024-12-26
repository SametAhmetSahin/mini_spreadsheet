use ast::AST;

use crate::raw_spreadsheet::{RawCell, RawSpreadSheet};
mod ast;

#[derive(Debug)]
struct Expression {}

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

enum ExprToken {
    CellName(String),
    Plus,
    Minus,
    Division,
    Multiply,
}

impl ExprToken {
    fn get_order(&self) -> usize{
        match &self {
            ExprToken::CellName(_) => 0,
            ExprToken::Plus => 1,
            ExprToken::Minus => 1,
            ExprToken::Division => 2,
            ExprToken::Multiply => 2,
        }
    }
}

struct ExpressionParser {
    index: usize,
    chars: Vec<char>,
}

impl ExpressionParser {
    fn new(chars: Vec<char>) -> Self {
        Self { index: 0, chars }
    }

    fn parse_expression(&mut self) -> Expression {
        let tokens = self.tokenize_expression();
        let ast = self.create_ast(tokens);

        Expression {}
    }

    fn tokenize_expression(&mut self) -> Vec<ExprToken> {
        self.skip_whitespace();
        let mut expr_tokens = Vec::new();
        while !self.is_done() {
            let token = match self.peek().expect("Should never fail") {
                '+' | '-' | '/' | '*' => self.parse_operator(),
                letter if letter.is_uppercase() => self.parse_cell_name().unwrap(),
                _unknown => todo!(),
            };

            expr_tokens.push(token);

            self.skip_whitespace();
        }

        expr_tokens
    }

    fn parse_cell_name(&mut self) -> Option<ExprToken> {
        // [A-Z]+\d+

        let mut is_valid = false;
        let mut cell_name = String::new();

        // Collect the uppercase letters
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_uppercase() {
                cell_name.push(ch);
                self.pop();
            } else {
                break;
            }
        }

        // Ensure there are letters
        if cell_name.is_empty() {
            return None;
        }

        // Collect the digits
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                cell_name.push(ch);
                self.pop();
                is_valid = true;
            } else {
                break;
            }
        }

        // Ensure the format was valid ``
        if !is_valid {
            return None;
        }

        Some(ExprToken::CellName(cell_name))
    }

    fn parse_operator(&mut self) -> ExprToken {
        match self.pop().expect("Shoud never fail") {
            '+' => ExprToken::Plus,
            '-' => ExprToken::Minus,
            '/' => ExprToken::Division,
            '*' => ExprToken::Multiply,
            _ => unreachable!(),
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn is_done(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn pop(&mut self) -> Option<&char> {
        let val = self.chars.get(self.index);
        self.index += 1;
        val
    }

    fn skip_whitespace(&mut self) -> bool {
        while !self.is_done() && (self.peek() == Some(&' ') || self.peek() == Some(&'\n')) {
            self.pop();
        }
        // Some error occured
        if self.is_done() {
            return false;
        }
        true
    }
    
    fn create_ast(&self, tokens: Vec<ExprToken>) -> AST {
        todo!()
    }
}

pub struct CellParser {}

impl CellParser {
    pub fn parse_raw(ss: RawSpreadSheet) -> ParsedSheet {
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

    fn parse_cell(rs: RawCell) -> CellType {
        let inner = rs.0;
        if inner.len() == 0 {
            return CellType::Empty;
        }

        match inner.chars().nth(0).unwrap() {
            '=' => Self::parse_expression(&inner).unwrap(),
            num if num.is_digit(10) => CellType::Number(inner.parse().unwrap()),
            _ => CellType::Text(inner),
        }
    }

    fn parse_expression(s: &str) -> Option<CellType> {
        let expr = ExpressionParser::new(s[1..].chars().collect()).parse_expression();

        Some(CellType::Expr(expr))
    }
}
