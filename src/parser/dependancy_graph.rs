use std::collections::HashMap;

use crate::raw_spreadsheet::Index;

use super::ParsedCell;

#[derive(Debug)]
pub struct DependancyGraph{ 
    /// Holds a topologically sorted graph of all dependencies
    inner : HashMap<Index, Vec<Index>>,
}

impl DependancyGraph {
    pub fn new(cells : HashMap<Index, ParsedCell>) -> Self{
        todo!()
    }

    pub fn get_all_dependants(&self, cell : &Index) -> Option<&Vec<Index>> {
        self.inner.get(&cell)
    }

    pub fn add_cell(&mut self, cell : &Index) {
        todo!();
    }
    pub fn remove_cell(&mut self, cell : &Index) {
        todo!();
    }

    pub fn mutate_cell(&mut self, cell: &Index){
        todo!();
    }
}