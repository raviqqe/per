mod item;
mod leaf;
mod node;

use self::item::Item;
use crate::radix_vec::leaf::Leaf;
use alloc::sync::Arc;

/// A radix vector.
#[derive(Clone, Default)]
pub struct RadixVec<T, const N: usize = 32> {
    root: Option<Arc<Item<T, N>>>,
    len: usize,
}

impl<T, const N: usize> RadixVec<T, N> {
    /// Creates a vector.
    #[must_use]
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns a length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no elements, or `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Pushes an element.
    pub fn push(&self, value: T) -> Self {
        Self {
            root: Some(Arc::new(if let Some(root) = &self.root {
                root.push(value)
            } else {
                Item::leaf(Leaf::singleton(value))
            })),
            len: self.len + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vec() {
        let _ = RadixVec::<isize>::new();
    }

    #[test]
    fn push() {
        let xs = RadixVec::<isize>::new();

        xs.push(42);
    }
}
