use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use rust_hypergraph::{
    directed_hypergraphs::DirectedBipartiteGraph,
    algorithms::bfs,
    Node
};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);