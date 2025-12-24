use super::leaf::Leaf;
use alloc::sync::Arc;
use core::mem::{ManuallyDrop, MaybeUninit};

union Item<T, const N: usize> {
    node: ManuallyDrop<Arc<Node<T, N>>>,
    leaf: ManuallyDrop<Arc<Leaf<T, N>>>,
}

#[derive(Debug)]
pub struct Node<T, const N: usize> {
    children: [MaybeUninit<Item<T, N>>; N],
    leaf: bool,
}

impl<T, const N: usize> Node<T, N> {
    pub fn new() -> Self {
        Self {
            children: [const { MaybeUninit::uninit() }; N],
            leaf: false,
        }
    }
}
