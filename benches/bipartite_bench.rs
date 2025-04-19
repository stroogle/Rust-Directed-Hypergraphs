use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use rust_hypergraph::directed_hypergraphs::DirectedBipartiteGraph;
use rust_hypergraph::algorithms::dfs;
use rust_hypergraph::Node;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let paths = fs::read_dir("./benches/data/balanced").unwrap();

    for path in paths {
        let file = path.unwrap().file_name().into_string().unwrap();

        let content = fs::read_to_string(
            format!("./benches/data/balanced/{}", file))
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

        c.bench_function(
        "dfs on bipartite",
        |b| b.iter(|| dfs(black_box(&g), 0))
        );
    }
//  todo!()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);