use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct Leaf<T, const N: usize> {
    children: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Leaf<T, N> {
    pub fn new() -> Self {
        Self {
            children: [const { MaybeUninit::uninit() }; N],
        }
    }
}
