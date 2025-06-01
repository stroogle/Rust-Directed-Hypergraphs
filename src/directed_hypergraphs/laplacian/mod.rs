use crate::{
    algorithms::interface::Graph,
    Node,
    directed_hypergraphs::DirectedBipartiteGraph
};

pub struct LaplacianDirectedHypergraph<T> {

    // pub graph: BipartiteGraph<T>

    pub matrix: Vec<Vec<usize>>,
    pub nodes: Vec<Node<T>>

}

impl<T: PartialEq + PartialOrd> LaplacianDirectedHypergraph<T> {

    pub fn new(graph: DirectedBipartiteGraph<T>) -> Self {
        let mut matrix: Vec<Vec<usize>> = graph.right_to_left_edges.clone();
        let nodes = graph.nodes;

        for (node_index, node) in graph.left_to_right_edges.iter().enumerate() {
            for (edge_index, _) in node.iter().enumerate() {
                if graph.left_to_right_edges[node_index][edge_index] == 1 {
                    matrix[edge_index][node_index] = 2;
                }
            }
        }

        Self {
            matrix,
            nodes
        }
    }

}

impl<T: PartialEq + PartialOrd> Graph for LaplacianDirectedHypergraph<T> {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize> {
        let tail_of_arcs: Vec<usize>;
        // let mut neighbors: HashSet<usize> = HashSet::new();
        let mut neighbors: Vec<usize> = Vec::new();

        tail_of_arcs = self.matrix
        .iter()
        .enumerate()
        .filter(|(_, value)| value[node_index] == 2)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

        
        for arc in tail_of_arcs.iter() {
            for (node, _) in self.matrix[*arc].iter().enumerate() {
                if self.matrix[*arc][node] == 1 {
                    // neighbors.insert(node);
                    neighbors.push(node);
                }
            }
        };

        // println!("{:?}", &neighbors);

        // neighbors.into_iter().collect()
        neighbors

    }

    fn count_out_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn count_in_degrees(&self, node_index: usize) -> usize {
        todo!()
    }

    fn node_count(&self) -> usize {
        self.matrix[0].len()
    }
}