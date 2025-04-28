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


#[cfg(test)]
mod tests {
    use rust_hypergraph::{algorithms::dfs, directed_hypergraphs::DirectedBipartiteGraph, Node};

    use super::*;

    #[test]
    fn balanced_traversable() {
        let folder_name = String::from("balanced");
        let files = get_file_names(&folder_name);

        for (_, file) in files.iter().enumerate() {
            let content = fs::read_to_string(
                format!("./benches/data/{}/{}", folder_name, file))
                .expect("Failed to read.");

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
            
            println!("File: {} saw {} node(s)", file, dfs(&g, 0));
        }
    }


    #[test]
    fn high_traversable() {
        let folder_name = String::from("high");
        let files = get_file_names(&folder_name);

        for (_, file) in files.iter().enumerate() {
            let content = fs::read_to_string(
                format!("./benches/data/{}/{}", folder_name, file))
                .expect("Failed to read.");

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
            
            println!("File: {} saw {} node(s)", file, dfs(&g, 0));
        }
    }

    #[test]
    fn medium_traversable() {
        let folder_name = String::from("medium");
        let files = get_file_names(&folder_name);

        for (_, file) in files.iter().enumerate() {
            let content = fs::read_to_string(
                format!("./benches/data/{}/{}", folder_name, file))
                .expect("Failed to read.");

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
            
            println!("File: {} saw {} node(s)", file, dfs(&g, 0));
        }
    }

    #[test]
    fn low_traversable() {
        let folder_name = String::from("low");
        let files = get_file_names(&folder_name);

        for (_, file) in files.iter().enumerate() {
            let content = fs::read_to_string(
                format!("./benches/data/{}/{}", folder_name, file))
                .expect("Failed to read.");

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
            
            println!("File: {} saw {} node(s)", file, dfs(&g, 0));
        }
    }
}