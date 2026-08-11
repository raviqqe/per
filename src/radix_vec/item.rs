use crate::radix_vec::{leaf::Leaf, node::Node};
use core::mem::ManuallyDrop;

pub union Item<T, const N: usize> {
    node: ManuallyDrop<Node<T, N>>,
    leaf: ManuallyDrop<Leaf<T, N>>,
}

impl<T, const N: usize> Item<T, N> {
    pub fn leaf(leaf: Leaf<T, N>) -> Self {
        Self {
            leaf: ManuallyDrop::new(leaf),
        }
    }

    pub fn push(&self, value: T) -> Self {
        todo!()
    }
}
