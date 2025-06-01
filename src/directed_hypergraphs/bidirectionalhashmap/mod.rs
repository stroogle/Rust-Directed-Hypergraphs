use std::collections::{HashSet, HashMap};

use crate::{algorithms::interface::Graph, HyperArc, Node};
use super::LaplacianDirectedHypergraph;

pub struct BidirectionalHashMap<T> {
    pub nodes: Vec<Node<T>>,
    pub arcs: Vec<HyperArc>,
    pub tails: HashMap<usize, HashSet<usize>>,
    pub heads: HashMap<usize, HashSet<usize>>
}

impl<T: PartialEq + PartialOrd> From<LaplacianDirectedHypergraph<T>> for BidirectionalHashMap<T> {
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

impl<T: PartialEq + PartialOrd> Graph for BidirectionalHashMap<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        if node_index < self.nodes.len() {
            let mut set_results: HashSet<usize> = HashSet::new();

            for (index, arc) in self.arcs.iter().enumerate() {
                if arc.head.contains(&node_index) {
                    set_results.insert(index + self.nodes.len());
                }
            }

            let results: Vec<usize> = set_results.into_iter().collect();

            results
        } else {
            let real_index = node_index - self.nodes.len();
            let results: Vec<usize> = self.arcs[real_index].tail.clone().into_iter().collect();

            results
        }
        
    }

    fn count_out_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn count_in_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.nodes.len() + self.arcs.len()
    }
}