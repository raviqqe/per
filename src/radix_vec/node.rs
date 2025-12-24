use super::item::Item;
use alloc::sync::Arc;
use core::mem::MaybeUninit;

pub struct Node<T, const N: usize> {
    children: [MaybeUninit<Arc<Item<T, N>>>; N],
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
