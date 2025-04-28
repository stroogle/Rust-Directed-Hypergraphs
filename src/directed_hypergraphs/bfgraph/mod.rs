use std::{collections::HashSet, hash::Hash};

use crate::{algorithms::interface::Graph, Node, HyperArc};

use super::DescriptiveDirectedHypergraph;

pub struct BFDirectedHypergraph<T> {
    pub nodes: Vec<Node<T>>,
    pub arcs: Vec<HyperArc>,
    // pub dummy_nodes: Vec<usize>
}

impl<T: PartialEq + PartialOrd> From<DescriptiveDirectedHypergraph<T>> for BFDirectedHypergraph<T> {
    fn from(value: DescriptiveDirectedHypergraph<T>) -> Self {

        let mut arcs: Vec<HyperArc> = vec![];

        for (index, arc) in value.arcs.iter().enumerate() {

            let dummy_node_index = value.nodes.len() + index;

            let mut dummy_set: HashSet<usize> = HashSet::new();

            dummy_set.insert(dummy_node_index);

            arcs.push(HyperArc {
                head: arc.head.clone(),
                tail: dummy_set.clone()
            });

            arcs.push(HyperArc {
                head: dummy_set.clone(),
                tail: arc.tail.clone()
            })

        }

        Self {
            nodes: value.nodes,
            arcs
        }
    }
}

impl<T: PartialEq + PartialOrd> Graph for BFDirectedHypergraph<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        let mut set_results: HashSet<usize> = HashSet::new();

        for arc in self.arcs.iter() {
            if arc.head.contains(&node_index) {
                set_results.extend(arc.tail.clone());
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