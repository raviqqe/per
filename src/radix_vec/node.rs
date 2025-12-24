use super::item::Item;
use core::mem::MaybeUninit;

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
