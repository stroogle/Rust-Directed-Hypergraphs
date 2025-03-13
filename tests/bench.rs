#[cfg(test)]
mod tests {

    use std::time::Instant;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use rust_hypergraph::{
        directed_hypergraphs::{
            DirectedBipartiteGraph,
            LaplacianDirectedHypergraph
        },
        algorithms::{
            dfs,
            bfs,
            interface::Graph
        },
        Node
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct Input {
        left_to_right_edges: Vec<Vec<usize>>,
        right_to_left_edges: Vec<Vec<usize>>
    }

    fn dfs_runner(n: usize, g: &dyn Graph) -> Vec<u128> {
        let mut result = Vec::new();

        for _ in 0..n {
            let now = Instant::now();
            {
                dfs(g, 0);
            }
            let elapsed = now.elapsed().as_nanos();
            result.push(elapsed);
        }

        result
    }

    #[test]
    fn bench_dfs() {

        let content = fs::read_to_string("./test.json").expect("Failed to read.");

        let i: Input = serde_json::from_str(&content).unwrap();

        let left_nodes: Vec<Node<usize>> = (0..i.left_to_right_edges.len())
        .map(|value| Node {value})
        .collect();

        let start = i.left_to_right_edges.len();

        let right_nodes: Vec<Node<usize>> = (start..(start+i.right_to_left_edges.len()))
        .map(|value| Node {value})
        .collect();

        let g = DirectedBipartiteGraph::new(left_nodes, right_nodes, i.left_to_right_edges, i.right_to_left_edges)
        .unwrap();

        println!("Bipartite: {:?}", dfs_runner(50, &g));

        let h = LaplacianDirectedHypergraph::new(g);

        println!("Laplacian: {:?}", dfs_runner(50, &h));

    }


}