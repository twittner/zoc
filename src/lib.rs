//! This crate contains a simple implementation of [Z-order curves][1] along
//! with `litmax` and `bigmin` calculations as described in the paper
//! ["Multidimensional Range Search in Dynamically Balanced Trees"][2] by H. Tropf
//! and H. Herzog (Angewandte Informatik 2/1981, pp. 71-77).
//!
//! [1]: https://en.wikipedia.org/wiki/Z-order_curve
//! [2]: http://www.vision-tools.com/h-tropf/multidimensionalrangequery.pdf

#![forbid(unsafe_code)]

mod z;
mod size;

pub mod search;

pub use z::{Bbox, Z};
pub use size::Size;

/// A type that has a [`Z`] value.
pub trait GetZ<const D: usize, T: Size<D>> {
    fn z(&self) -> &Z<D, T>;
}

impl<const D: usize, T: Size<D>> GetZ<D, T> for Z<D, T> {
    fn z(&self) -> &Z<D, T> {
        self
    }
}
