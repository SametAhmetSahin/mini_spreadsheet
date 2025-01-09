use parser::{
    ast_resolver::{ASTResolver, VarContext},
    dependancy_graph::{DependancyGraph, TopologicalSort},
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
        self.get_computed(index)
    }
}

impl SpreadSheet {
    fn parse_and_add_raw(&mut self, index: Index, mut cell: Cell) {
        CellParser::parse_cell(&mut cell);

        self.add_dependencies(index, &cell);

        self.cells.insert(index, cell);
    }

    /// Adds the dependency graph for a cell based on its parsed representation.
    fn add_dependencies(&mut self, index: Index, cell: &Cell) {
        if let Some(Ok(ParsedCell::Expr(Expression { ref dependencies, .. }))) = cell.parsed_representation {
            self.dependencies.change_node(index, dependencies);
        } else {
            self.dependencies.change_node(index, &vec![]);
        }
    }

    /// Updates the dependency graph for a cell based on its parsed representation.
    fn update_dependencies(&mut self, index: Index, cell: &Cell) {
        if let Some(Ok(ParsedCell::Expr(Expression { ref dependencies, .. }))) = cell.parsed_representation {
            self.dependencies.add_node(index, dependencies);
        } else {
            self.dependencies.add_node(index, &vec![]);
        }
    }

    /// Computes the value of a cell based on its parsed representation.
    fn compute_cell(&self, cell: &Cell) -> Option<Result<Value, ComputeError>> {
        match cell.parsed_representation {
            Some(Ok(ParsedCell::Expr(ref expr))) => Some(ASTResolver::resolve(&expr.ast, self)),
            Some(Ok(ParsedCell::Value(ref value))) => Some(Ok(value.clone())),
            Some(Err(ref e)) => Some(Err(ComputeError::ParseError(e.0.clone()))),
            None => None,
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
        let TopologicalSort { sorted, cycles } = self.dependencies.topological_sort();

        for idx in sorted {
            let cell = self.cells.get(&idx).expect("should not fail");
            if !cell.needs_compute {
                continue;
            }
            let computed = self.compute_cell(cell);

            let cell = self.cells.get_mut(&idx).expect("should not fail");
            cell.computed_value = computed;
            cell.needs_compute = false
        }

        for idx in cycles {
            let cell = self.cells.get_mut(&idx).expect("should not fail");
            if !cell.needs_compute {
                continue;
            }
            cell.computed_value = Some(Err(ComputeError::Cycle));
        }
    }

    pub fn get_computed(&self, index: Index) -> Option<Result<Value, ComputeError>> {
        self.cells.get(&index)?.computed_value.clone()
    }

    
    pub fn get_text(&self, index: Index)-> String{
        match self.get_computed(index){
            Some(value) => match value {
                Ok(inner) => inner.to_string(),
                Err(err) => err.to_string(),
            },
            None => String::new(),
        }
    }

    pub fn add_cell_and_compute(&mut self, index: Index, raw: String) {
        let mut cell = Cell::from_raw(raw);
        CellParser::parse_cell(&mut cell);

        self.add_dependencies(index, &cell);

        cell.computed_value = self.compute_cell(&cell);
        cell.needs_compute = false;
        self.cells.insert(index, cell);

        let mut need_compute = false;
        for dep in self.dependencies.get_all_dependants(index) {
            if let Some(cell) = self.cells.get_mut(&dep) {
                cell.needs_compute = true;
                need_compute = true;
            }
        }
        if need_compute {
            self.compute_all();
        }
    }

    pub fn remove_cell(&mut self, index: Index) {
        // Remove from dependencies
        self.dependencies.remove_node(index);
        self.cells.remove(&index);

        let mut need_compute = false;
        for dep in self.dependencies.get_all_dependants(index) {
            if let Some(cell) = self.cells.get_mut(&dep) {
                cell.needs_compute = true;
                need_compute = true;
            }
        }
        if need_compute {
            self.compute_all();
        }
    }

    pub fn mutate_cell(&mut self, index: Index, new_raw: String) {
        let mut new_cell = Cell::from_raw(new_raw);
        CellParser::parse_cell(&mut new_cell);
        new_cell.computed_value = self.compute_cell(&new_cell);
        new_cell.needs_compute = false;

        self.update_dependencies(index, &new_cell);

        let cell = self
            .cells
            .get_mut(&index)
            .expect("Expected valid index for mutate cell");
        *cell = new_cell;


        let mut need_compute = false;
        for dep in self.dependencies.get_all_dependants(index) {
            if let Some(cell) = self.cells.get_mut(&dep) {
                cell.needs_compute = true;
                need_compute = true;
            }
        }
        if need_compute {
            self.compute_all();
        }
    }
    
    pub fn get_raw(&self, index: &Index) -> Option<&str> {
        Some(&self.cells.get(&index)?.raw_representation)
    }
}
