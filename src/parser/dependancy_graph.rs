use std::{collections::HashMap, vec};
use std::hash::Hash;

use crate::raw_spreadsheet::Index;

use super::{Expression, ParsedCell};

#[derive(Debug)]
pub struct DependancyGraph {
    // Calculating one key should allow us to calculate the its values
    inner: HashMap<Index, Vec<Index>>,
}

impl DependancyGraph {
    pub fn new(parsed_cells: HashMap<Index, ParsedCell>) -> Self {
        let mut inner: HashMap<Index, Vec<Index>> = HashMap::new();


        for (key, value) in parsed_cells {
            if let ParsedCell::Expr(Expression { dependencies, .. }) = value {
                for dependency in dependencies {
                    inner.entry(dependency).or_default().push(key);
                }
            }
        }

        DependancyGraph { inner }
    }

    pub fn add_cell(&mut self, cell : Index, dependencies : Vec<Index>){
        for dependency in dependencies {
            self.inner.entry(dependency).or_default().push(cell);
        }
    }

    pub fn remove_cell(&mut self, cell : &Index){
        self.inner.remove(&cell);
    }
    
}
