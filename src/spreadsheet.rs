use parser::{
    ast_resolver::{ASTResolver, VarContext},
    dependancy_graph::DependancyGraph,
    CellParser,
};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use crate::common_types::{Cell, Expression, Index, ParsedCell, Value};
mod parser;

#[derive(Debug, Default)]
pub struct SpreadSheet {
    pub cells: HashMap<Index, Cell>,
    dependencies: DependancyGraph,
}

impl VarContext for SpreadSheet {
    fn get_variable(&self, index: Index) -> Option<Value> {
        self.cells.get(&index)?.computed_value.clone()
    }
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

    pub fn compute_cell(&self, cell: &Cell) -> Option<Value> {
        if let Some(parsed_cell) = &cell.parsed_representation {
            match parsed_cell {
                ParsedCell::Expr(expression) => Some(ASTResolver::resolve(&expression.ast, self)),
                ParsedCell::Value(value) => Some(value.clone()),
            }
        } else {
            None
        }
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

    pub fn compute_all(&mut self) {
        let mut indices_ordered : Vec<Index>= self.cells.keys().cloned().collect::<Vec<Index>>();
        indices_ordered.sort();

        for k in indices_ordered{
            let cell = self.cells.get(&k).unwrap();
            let computed = self.compute_cell(cell);

            let cell = self.cells.get_mut(&k).unwrap();
            cell.computed_value = computed;
        }
    }
}
