use std::{collections::HashMap, vec};

use crate::spreadsheet::Index;

#[derive(Debug, Default)]
pub struct DependancyGraph {
    // Calculating one key should allow us to calculate the its values
    inner: HashMap<Index, Vec<Index>>,
}

#[derive(Debug)]
pub struct TopologicalSort {
    pub sorted: Vec<Index>,
    pub cycles: Vec<Index>,
}

impl DependancyGraph {
    pub fn add_cell(&mut self, cell: Index, dependencies: &Vec<Index>) {
        self.inner.entry(cell).or_default();

        for dependency in dependencies {
            self.inner.entry(*dependency).or_default().push(cell);
        }
    }

    pub fn topological_sort(&self) -> TopologicalSort {
        let mut in_degree: HashMap<Index, usize> = HashMap::new();
        let mut zero_in_degree: Vec<Index> = vec![];
        let mut sorted: Vec<Index> = vec![];
        let mut cycles: Vec<Index> = vec![];

        // Calculate in-degrees for all nodes
        for (node, dependents) in &self.inner {
            in_degree.entry(*node).or_insert(0); // Ensure all nodes exist in the map
            for dependent in dependents {
                *in_degree.entry(*dependent).or_insert(0) += 1;
            }
        }

        // Find all nodes with zero in-degree
        for (node, degree) in &in_degree {
            if *degree == 0 {
                zero_in_degree.push(*node);
            }
        }

        // Process nodes with zero in-degree
        while let Some(node) = zero_in_degree.pop() {
            sorted.push(node);

            // Decrease the in-degree of all its dependents
            if let Some(dependents) = self.inner.get(&node) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            zero_in_degree.push(*dependent);
                        }
                    }
                }
            }
        }

        // Collect nodes with non-zero in-degree as cycles
        for (node, degree) in in_degree {
            if degree > 0 {
                cycles.push(node);
            }
        }

        TopologicalSort { sorted, cycles }
    }
}
