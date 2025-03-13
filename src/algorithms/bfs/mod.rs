use crate::algorithms::interface::Graph;
use std::collections::VecDeque;

pub fn bfs(graph: &dyn Graph, start_node_index: usize) -> i32 {

    let node_count: usize = graph.node_count();

    let mut visited: Vec<bool> = vec![false; node_count];

    let mut queue = VecDeque::new();

    queue.push_back(start_node_index);

    while let Some(node_index) = queue.pop_front() {

        visited[node_index] = true;
        // print!("Just visited {:?}", node_index);

        let neighbors: Vec<usize> = graph.get_neighbors(node_index);

        for neighbor in neighbors.iter() {
            if !visited[*neighbor] {
                queue.push_back(*neighbor);
            }
        }

    }

    1
}