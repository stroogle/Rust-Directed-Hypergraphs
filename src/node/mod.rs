#[derive(Debug)]
pub struct Node<T> {
    pub value: T
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}