use parser::{
    ast_resolver::{ASTResolver, VarContext},
    dependancy_graph::DependancyGraph,
    CellParser,
};
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use crate::common_types::{Cell, ComputeError, Expression, Index, ParsedCell, Value};
mod parser;

#[derive(Debug, Default)]
pub struct SpreadSheet {
    pub cells: HashMap<Index, Cell>,
    dependencies: DependancyGraph,
}

impl VarContext for SpreadSheet {
    fn get_variable(&self, index: Index) -> Option<Result<Value, ComputeError>> {
        self.cells.get(&index)?.computed_value.clone()
    }
}

impl SpreadSheet {
    pub fn parse_and_add_raw(&mut self, index: Index, mut cell: Cell) {
        CellParser::parse_cell(&mut cell);

        if let Some(Ok(ParsedCell::Expr(Expression {
            ref dependencies, ..
        }))) = cell.parsed_representation
        {
            self.dependencies.add_cell(index, dependencies);
        }
        self.cells.insert(index, cell);
    }

    pub fn compute_cell(&self, cell: &Cell) -> Option<Result<Value, ComputeError>> {
        if let Some(parsed_cell) = &cell.parsed_representation {
            match parsed_cell {
                Ok(inner) => match inner {
                    ParsedCell::Expr(expression) => {
                        Some(ASTResolver::resolve(&expression.ast, self))
                    }
                    ParsedCell::Value(value) => Some(Ok(value.clone())),
                },
                Err(e) => Some(Err(ComputeError::ParseError(e.0.clone()))),
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
                spreadsheet.parse_and_add_raw(Index { x, y }, Cell::from_raw(cell));
            }
        }

        spreadsheet
    }

    pub fn compute_all(&mut self) {
        let mut indices_ordered: Vec<Index> = self.cells.keys().cloned().collect::<Vec<Index>>();
        indices_ordered.sort();

        for k in indices_ordered {
            let cell = self.cells.get(&k).expect("should not fail");
            let computed = self.compute_cell(cell);

            let cell = self.cells.get_mut(&k).expect("should not fail");
            cell.computed_value = computed;
        }
    }
}
