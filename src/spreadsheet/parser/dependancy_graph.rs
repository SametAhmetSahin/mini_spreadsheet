use std::{collections::{HashMap, HashSet}, vec};

use crate::spreadsheet::Index;

#[derive(Debug, Default)]
pub struct DependancyGraph {
    allows_compute: HashMap<Index, Vec<Index>>, // Given a key return nodes this node allows for compute
}

#[derive(Debug)]
pub struct TopologicalSort {
    pub sorted: Vec<Index>,
    pub cycles: Vec<Index>,
}

impl DependancyGraph {
    pub fn add_node(&mut self, idx: Index, cel_depends_on: &Vec<Index>) {
        self.allows_compute.entry(idx).or_default();

        for dependency in cel_depends_on {
            self.allows_compute.entry(*dependency).or_default().push(idx);
        }
    }

    pub fn topological_sort(&self) -> TopologicalSort {
        let mut in_degree: HashMap<Index, usize> = HashMap::new();
        let mut zero_in_degree: Vec<Index> = vec![];
        let mut sorted: Vec<Index> = vec![];
        let mut cycles: Vec<Index> = vec![];

        // Calculate in-degrees for all nodes
        for (node, dependents) in &self.allows_compute {
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
            if let Some(dependents) = self.allows_compute.get(&node) {
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

    pub fn remove_node(&mut self, index: Index) {
        // Remove all edges going to the given node
        for dependants in self.allows_compute.values_mut() {
            dependants.retain(|&x| x != index);
        }
        
    }

    pub fn change_node(&mut self, index: Index, dependencies: &Vec<Index>) {
        self.remove_node(index);
        // Re-add the node with the new dependencies
        self.add_node(index, dependencies);
    }

    /// Return all nodes that depend on this
    pub fn get_all_dependants(&self, index: Index) -> Vec<Index> {        
        let mut result = Vec::new();
        let mut to_process = vec![index];

        while let Some(cell) = to_process.pop() {
            if let Some(dependants) = self.allows_compute.get(&cell) {
                for dependant in dependants {
                    if !result.contains(dependant) {
                        result.push(*dependant);
                        to_process.push(*dependant);
                    }
                }
            }
        }

        result
    }
}
