use crate::algorithms::interface::Graph;
use crate::Node;
use std::cmp::Ordering;

pub struct DirectedBipartiteGraph<T> {
    pub nodes: Vec<Node<T>>,
    pub left_to_right_edges: Vec<Vec<usize>>,
    pub right_to_left_edges: Vec<Vec<usize>>
}

impl<T: PartialEq + PartialOrd> DirectedBipartiteGraph<T> {
    /// |left_nodes| > 1
    /// |right_nodes| >= 1
    /// left_nodes.len() == left_to_right_edges.len()
    /// right_nodes.len() == left_to_right_edges[n].len()
    /// right_nodes.len() == right_to_left_edges.len()
    /// left_nodes.len() == right_to_left_edges[n].len()
    pub fn new(
        mut left_nodes: Vec<Node<T>>,
        mut right_nodes: Vec<Node<T>>,
        left_to_right_edges: Vec<Vec<usize>>,
        right_to_left_edges: Vec<Vec<usize>>
    ) -> Result<Self, String> {
        
        let number_of_left_nodes: usize = left_nodes.len();
        let number_of_right_nodes: usize = right_nodes.len();
        let number_of_left_to_right_sources: usize = left_to_right_edges.len();
        let number_of_right_to_left_sources: usize = right_to_left_edges.len();

        if number_of_left_nodes < 2 {
            return Err(String::from("Size of left nodes cannot be less than 2."));
        }
        
        if number_of_right_nodes < 1 {
            return Err(String::from("Size of right nodes cannot be less than 1."));
        }

        if number_of_left_nodes != number_of_left_to_right_sources {
            return Err(String::from("Number of left nodes must equal the size of left_to_right_edges.len()"));
        }

        if number_of_right_nodes != number_of_right_to_left_sources {
            return Err(String::from("Number of right nodes must equal the size of right_to_left_edges.len()"));
        }

        let number_of_left_to_right_targets: usize = left_to_right_edges[0].len();
        let number_of_right_to_left_targets: usize = right_to_left_edges[0].len();

        if number_of_left_nodes != number_of_right_to_left_targets {
            return Err(String::from("Number of left nodes must equal the size of right_to_left_edges[n].len()"));
        }

        if number_of_right_nodes != number_of_left_to_right_targets {
            return Err(String::from("Number of right nodes must equal the size of left_to_right_edges[n].len()"));
        }

        left_nodes.append(&mut right_nodes);

        Ok(Self {
            nodes: left_nodes,
            left_to_right_edges,
            right_to_left_edges
        })

    }

    pub fn get_out_neighbours(&self, node: usize) -> Vec<usize> {
        let node_index: usize;
        let edges: &Vec<Vec<usize>>;
        let offset: usize;

        match node.cmp(&self.left_to_right_edges.len()) {
            Ordering::Less => {
                node_index = node;
                edges = &self.left_to_right_edges;
                offset = self.left_to_right_edges.len();
            },
            _ => {
                node_index = node - self.left_to_right_edges.len();
                edges = &self.right_to_left_edges;
                offset = 0;
            }
        }

        edges[node_index]
        .iter()
        .enumerate()
        .filter(|&(_, value)| *value == 1)
        .map(|(idx, _)| idx + offset)
        .collect()
    }

    pub fn get_node(&self, node_index: usize) -> &Node<T> {
        let mut node = node_index;

        if node_index >= self.nodes.len() {
            node = self.nodes.len() - 1;
        }

        &self.nodes[node]
    }
}

impl<T: PartialEq + PartialOrd> Graph for DirectedBipartiteGraph<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        self.get_out_neighbours(node_index)
    }

    fn count_out_degrees(&self, node_index: usize) -> usize {
        self.get_out_neighbours(node_index).len()
    }

    fn count_in_degrees(&self, _node_index: usize) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}