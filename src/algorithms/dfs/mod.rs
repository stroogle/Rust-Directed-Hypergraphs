use crate::algorithms::interface::Graph;

pub fn dfs(graph: &dyn Graph, start_node_index: usize) -> i32 {
    let node_count: usize = graph.node_count();

    let mut visited: Vec<bool> = vec![false; node_count];

    let mut stack: Vec<usize> = vec![start_node_index; 1];

    while let Some(node_index) = stack.pop() {

        visited[node_index] = true;
        // println!("{:?}", node_index);

        let neighbors: Vec<usize> = graph.get_neighbors(node_index);

        for neighbor in neighbors.iter() {
            if !visited[*neighbor] && !stack.contains(neighbor){
                stack.push(*neighbor);
            }
        }

    }

    1
}