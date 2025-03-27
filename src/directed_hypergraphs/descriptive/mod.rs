use std::collections::HashSet;

use crate::{algorithms::interface::Graph, HyperArc, Node};
use super::LaplacianDirectedHypergraph;

pub struct DescriptiveDirectedHypergraph<T> {
    pub nodes: Vec<Node<T>>,
    pub arcs: Vec<HyperArc>
}

impl<T: PartialEq + PartialOrd> From<LaplacianDirectedHypergraph<T>> for DescriptiveDirectedHypergraph<T> {
    fn from(value: LaplacianDirectedHypergraph<T>) -> Self {

        let mut arcs: Vec<HyperArc> = vec![];

        for arc in value.matrix.iter() {
            let head: HashSet<usize>;
            let tail: HashSet<usize>;

            head = arc.iter()
            .enumerate()
            .filter(|(_, value)| **value == 1)
            .map(|(index, _)| index)
            .collect();

            tail = arc.iter()
            .enumerate()
            .filter(|(_, value)| **value == 2)
            .map(|(index, _)| index)
            .collect();


            arcs.push(HyperArc {
                head,
                tail
            })
        }

        Self {
            nodes: value.nodes,
            arcs
        }
    }
}

impl<T: PartialEq + PartialOrd> Graph for DescriptiveDirectedHypergraph<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        let mut set_results: HashSet<usize> = HashSet::new();

        for arc in self.arcs.iter() {
            if arc.head.contains(&node_index) {
                set_results.extend(arc.head.clone());
            }
        }

        let results: Vec<usize> = set_results.into_iter().collect();

        results
    }

    fn count_out_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn count_in_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}