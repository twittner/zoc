#![forbid(unsafe_code)]

mod z;
mod size;

pub mod search;

pub use z::{Bbox, Z};
pub use size::Size;

pub trait GetZ<const D: usize, T: Size<D>> {
    fn z(&self) -> Z<D, T>;
}

impl<const D: usize, T: Size<D>> GetZ<D, T> for Z<D, T> {
    fn z(&self) -> Z<D, T> {
        *self
    }
}
