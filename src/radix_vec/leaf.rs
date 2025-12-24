use core::{mem::MaybeUninit, ptr::write};

pub struct Leaf<T, const N: usize> {
    children: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Leaf<T, N> {
    pub fn new() -> Self {
        Self {
            children: [const { MaybeUninit::uninit() }; N],
        }
    }

    pub fn singleton(value: T) -> Self {
        let mut children = [const { MaybeUninit::uninit() }; N];

        unsafe { write(&mut children[0], MaybeUninit::new(value)) };

        Self { children }
    }
}
