use std::collections::{HashMap, HashSet};

use crate::{algorithms::interface::Graph, HyperArc, Node};
use super::DescriptiveDirectedHypergraph;

pub struct BidirectionalHashMap<T> {
    pub nodes: Vec<Node<T>>,
    pub arcs: Vec<HyperArc>,
    pub tails: HashMap<usize, HashSet<usize>>,
    pub heads: HashMap<usize, HashSet<usize>>
}

impl<T: PartialEq + PartialOrd> From<DescriptiveDirectedHypergraph<T>> for BidirectionalHashMap<T> {
    fn from(value: DescriptiveDirectedHypergraph<T>) -> Self {

        let mut tails: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut heads: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (index, arc) in value.arcs.iter().enumerate() {
            let dummy_node_index = value.nodes.len() + index;

            heads.insert(dummy_node_index, HashSet::new());

            for node in &arc.tail {
                if !tails.contains_key(node) {
                    tails.insert(*node, HashSet::new());
                }

                if let Some(v) = tails.get_mut(node) {
                    v.insert(dummy_node_index);
                }
            }

            for node in &arc.head {
                if let Some(v) = heads.get_mut(&dummy_node_index) {
                    v.insert(*node);
                }
            }
        }


        Self {
            nodes: value.nodes,
            arcs: value.arcs,
            tails,
            heads
        }
        
    }
}

impl<T: PartialEq + PartialOrd> Graph for BidirectionalHashMap<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {

        let results: Vec<usize>;

        if self.tails.contains_key(&node_index) {
            // results = self.tails.get(&node_index).into_iter().collect();
            results = match self.tails.get(&node_index) {
                Some(v) => v.clone().into_iter().collect(),
                _ => Vec::new()
            };
            results
        } else {
            results = match self.heads.get(&node_index) {
                Some(v) => v.clone().into_iter().collect(),
                _ => Vec::new()
            };
            results
        }
        
    }

    fn count_out_degrees(&self, _node_index: usize) -> usize {
        todo!()
    }

    fn count_in_degrees(&self, _node_index: usize) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.nodes.len() + self.arcs.len()
    }
}