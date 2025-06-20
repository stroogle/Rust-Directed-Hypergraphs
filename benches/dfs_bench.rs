use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId
};
use rust_hypergraph::{
    algorithms::dfs, directed_hypergraphs::{
        BFDirectedHypergraph, BidirectionalHashMap, DescriptiveDirectedHypergraph, DirectedBipartiteGraph, LaplacianDirectedHypergraph
    }, Node
};
use std::fs;

pub fn get_file_names(folder: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    let dir = format!("./benches/data/{}", folder);
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let file = path.unwrap().file_name().into_string().unwrap();
        files.push(file);
    }

    files
}

pub fn run_benchmarks(folder_name: &str, group_name: &str, c: &mut Criterion) {
    let files = get_file_names(&folder_name);
    let mut group = c.benchmark_group(group_name);

    for (index, file) in files.iter().enumerate() {
        let content = fs::read_to_string(
            format!("./benches/data/{}/{}", folder_name, file))
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

        group.bench_with_input(
            BenchmarkId::new("bipartite", format!("Graph #{}", index)),
            &g,
            |b, g| b.iter(|| dfs(g, 0))
        );

        let h = LaplacianDirectedHypergraph::new(g);

        group.bench_with_input(
            BenchmarkId::new("laplacian", format!("Graph #{}", index)),
            &h,
            |b, h| b.iter(|| dfs(h, 0))
        );

        let j = DescriptiveDirectedHypergraph::from(h);
        let j2 = j.clone();

        group.bench_with_input(
            BenchmarkId::new("Descriptive", format!("Graph #{}", index)),
            &j,
            |b, j| b.iter(|| dfs(j, 0))
        );

        let k = BFDirectedHypergraph::from(j);

        group.bench_with_input(
            BenchmarkId::new("BF", format!("Graph #{}", index)),
            &k,
            |b, k| b.iter(|| dfs(k, 0))
        );

        let l = BidirectionalHashMap::from(j2);

        group.bench_with_input(
            BenchmarkId::new("Bidirectional", format!("Graph #{}", index)),
            &l,
            |b, l| b.iter(|| dfs(l, 0))
        );
    }
}

pub fn balanced_dfs_benchmark(c: &mut Criterion) {
    run_benchmarks("balanced", "balanced_dfs", c);
}

pub fn high_dfs_benchmark(c: &mut Criterion) {
    run_benchmarks("high", "high_dfs", c);
}

pub fn med_dfs_benchmark(c: &mut Criterion) {
    run_benchmarks("medium", "med_dfs", c);
}

pub fn low_dfs_benchmark(c: &mut Criterion) {
    run_benchmarks("low", "low_dfs", c);
}

criterion_group!(
    benches,
    balanced_dfs_benchmark,
    high_dfs_benchmark,
    med_dfs_benchmark,
    low_dfs_benchmark
);
criterion_main!(benches);