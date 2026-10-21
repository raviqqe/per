#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod radix_vec;

pub use radix_vec::*;

// TODO Remove this dummy function.
/// Adds two numbers.
pub const fn add(x: usize, y: usize) -> usize {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
