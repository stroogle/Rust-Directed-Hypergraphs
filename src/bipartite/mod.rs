#[derive(Debug)]
pub struct DirectedBipartite {
    pub left_column: usize,
    pub right_column: usize,
    pub edges: Vec<Vec<usize>>    
}

impl DirectedBipartite {
    pub fn new(left_column: usize, right_column: usize) -> Self {
        let edges = vec![vec![0; left_column + right_column]; left_column + right_column];
        Self {
            left_column,
            right_column,
            edges
        }
    }

    pub fn add_edge(&mut self, source: usize, destination: usize) -> Result<(), &'static str> {

        if source >= self.left_column && destination >= self.left_column {
            return Err("Invalid edge for a bipartite graph... Both vertices would be in the right column.");
        }

        if source < self.left_column && destination < self.left_column {
            return Err("Invalid edge for a bipartite right... Both vertices would be in the left column.");
        }

        self.edges[source][destination] = 1;
        Ok(())
    }

    pub fn remove_edge(&mut self, source: usize, destination: usize) -> Result<(), &'static str> {
        if source >= self.left_column + self.right_column || destination >= self.left_column + self.right_column {
            return Err("Invalid node coordinates.");
        }

        self.edges[source][destination] = 0;

        Ok(())
    }

    pub fn get_destinations(&self, vertex: usize) -> Vec<usize> {
        if vertex >= self.left_column + self.right_column {
            return vec![];
        }

        self.edges[vertex]
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(index, _)| index)
        .collect()
    }

    pub fn get_sources(&self, vertex: usize) -> Vec<usize> {
        if vertex >= self.left_column + self.right_column {
            return vec![];
        }

        self.edges
        .iter()
        .enumerate()
        .filter(|(_, val)| val[vertex] == 1)
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_bipartite() {
        let graph = DirectedBipartite::new(4, 6);

        assert_eq!(graph.left_column, 4);
        assert_eq!(graph.right_column, 6);
        print!("{:?}", graph.edges);
    }

    #[test]
    fn add_edge() {
        let mut graph = DirectedBipartite::new(4, 6);

        let mut res = graph.add_edge(2, 9);

        match res {
            Ok(()) => assert_eq!(graph.edges[2][9], 1),
            _ => panic!("Failed test"),
        }

        res = graph.add_edge(1, 1);

        match res {
            Err(e) => assert_eq!(e, "Invalid edge for a bipartite right... Both vertices would be in the left column."),
            _ => panic!("Failed test"),
        }

        res = graph.add_edge(9, 7);

        match res {
            Err(e) => assert_eq!(e, "Invalid edge for a bipartite graph... Both vertices would be in the right column."),
            _ => panic!("Failed test"),
        }

        print!("{:?}", graph.edges);
    }

    #[test]
    fn remove_edge() {
        let mut graph = DirectedBipartite::new(4, 6);

        let mut res = graph.add_edge(2, 9);

        
        match res {
            Ok(()) => assert_eq!(graph.edges[2][9], 1),
            _ => panic!("Failed test"),
        }
        
        res = graph.remove_edge(2, 9);

        match res {
            Err(_) => panic!("Failed to remove valid index"),
            Ok(_) => assert_eq!(graph.edges[2][9], 0),
        }

        match graph.remove_edge(10, 12) {
            Err(_) => assert!(true),
            _ => assert!(false)
        }

        print!("{:?}", graph.edges);
    }

    #[test]
    fn get_destinations() {
        let mut graph = DirectedBipartite::new(4, 6);

        graph.add_edge(2, 5).unwrap();
        graph.add_edge(2, 4).unwrap();
        graph.add_edge(2, 9).unwrap();

        assert_eq!(graph.get_destinations(2), vec![4, 5, 9]);
        assert_eq!(graph.get_destinations(1), vec![]);
        assert_eq!(graph.get_destinations(9), vec![]);

        graph.add_edge(9, 0).unwrap();
        graph.add_edge(9, 1).unwrap();
        graph.add_edge(9, 2).unwrap();
        assert_eq!(graph.get_destinations(9), vec![0, 1, 2]);

        assert_eq!(graph.get_destinations(2000), vec![]);

    }

    #[test]
    fn get_sources() {
        let mut graph = DirectedBipartite::new(4, 6);

        graph.add_edge(2, 5).unwrap();
        graph.add_edge(2, 4).unwrap();
        graph.add_edge(2, 9).unwrap();

        assert_eq!(graph.get_sources(5), vec![2]);
        assert_eq!(graph.get_sources(4), vec![2]);
        assert_eq!(graph.get_sources(9), vec![2]);

        graph.add_edge(9, 0).unwrap();
        graph.add_edge(9, 1).unwrap();
        graph.add_edge(9, 2).unwrap();
        assert_eq!(graph.get_sources(2), vec![9]);

        graph.add_edge(8, 0).unwrap();
        graph.add_edge(7, 0).unwrap();
        graph.add_edge(6, 0).unwrap();
        assert_eq!(graph.get_sources(0), vec![6, 7, 8, 9]);

        assert_eq!(graph.get_sources(2000), vec![]);

    }
}