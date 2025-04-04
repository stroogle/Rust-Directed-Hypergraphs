#[cfg(test)]
mod tests {

    use std::time::Instant;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use rust_hypergraph::{
        directed_hypergraphs::{
            DirectedBipartiteGraph,
            LaplacianDirectedHypergraph,
            DescriptiveDirectedHypergraph,
            BFDirectedHypergraph
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

        let paths = fs::read_dir("./data/highly_connected").unwrap();

        for path in paths {
            let file = path.unwrap().file_name().into_string().unwrap();

            println!("{}", file);

            let content = fs::read_to_string(
                format!("./data/highly_connected/{}", file))
                .expect("Failed to read."
            );

            let i: Vec<Vec<Vec<usize>>> = serde_json::from_str(&content).unwrap();

            let left_nodes: Vec<Node<usize>> = (0..i[0].len())
            .map(|value| Node {value})
            .collect();

            let start = i[0].len();

            let right_nodes: Vec<Node<usize>> = (start..(start+i[1].len()))
            .map(|value| Node {value})
            .collect();

            let g = DirectedBipartiteGraph::new(left_nodes, right_nodes, i[0].clone(), i[1].clone())
            .unwrap();

            println!("Bipartite: {:?}", dfs_runner(50, &g));

            let h = LaplacianDirectedHypergraph::new(g);

            println!("Laplacian: {:?}", dfs_runner(1, &h));

            let j = DescriptiveDirectedHypergraph::from(h);

            println!("Descriptive: {:?}", dfs_runner(1, &j));

            let k = BFDirectedHypergraph::from(j);

            println!("BF-Graph: {:?}", dfs_runner(1, &k));
        }

        


    }


}