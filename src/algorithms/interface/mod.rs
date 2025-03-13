pub trait Graph {
    fn get_neighbors(&self, node_index: usize) -> Vec<usize>;
    fn count_out_degrees(&self, node_index: usize) -> usize;
    fn count_in_degrees(&self, node_index: usize) -> usize;
    fn node_count(&self) -> usize;
}