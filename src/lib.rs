#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(test)]
extern crate std;

/// Adds two numbers.
pub fn add(x: usize, y: usize) -> usize {
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
