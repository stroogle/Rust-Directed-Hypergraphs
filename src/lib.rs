mod node;
mod hyperarc;
pub mod algorithms;
pub mod directed_hypergraphs;
pub use node::Node;
pub use hyperarc::HyperArc;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
