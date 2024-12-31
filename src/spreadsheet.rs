use parser::{dependancy_graph::DependancyGraph, CellParser};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use crate::common_types::{Cell, Expression, Index, ParsedCell};
mod parser;

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
