use std::{collections::HashMap, vec};

use crate::spreadsheet::Index;

use super::{Expression, ParsedCell};

#[derive(Debug, Default)]
pub struct DependancyGraph {
    // Calculating one key should allow us to calculate the its values
    inner: HashMap<Index, Vec<Index>>,
}

impl DependancyGraph {
    pub fn add_cell(&mut self, cell: Index, dependencies: &Vec<Index>) {
        for dependency in dependencies {
            self.inner.entry(*dependency).or_default().push(cell);
        }
    }

    pub fn remove_cell(&mut self, cell: &Index) {
        self.inner.remove(&cell);
    }
}
