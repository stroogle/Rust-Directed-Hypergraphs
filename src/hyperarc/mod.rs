use std::collections::HashSet;

pub struct HyperArc {
    pub head: HashSet<usize>,
    pub tail: HashSet<usize>
}