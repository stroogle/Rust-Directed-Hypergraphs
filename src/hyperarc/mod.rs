use std::collections::HashSet;

pub struct HyperArc {
    pub head: HashSet<usize>,
    pub tail: HashSet<usize>
}

impl Clone for HyperArc {
    fn clone(&self) -> Self {
        Self { head: self.head.clone(), tail: self.tail.clone() }
    }
}