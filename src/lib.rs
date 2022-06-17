//! cucoqu.rlib — convert cubic, conic, and quadratic bezier segments between one another
//!
//! TODO: arbitrarily raise degree of an n-degree bezier segment replaces a lot of needlessly used
//! Rust-unsafe Skia code

mod coeffs;
mod p3d;
pub use p3d::{Point3, Point3Like};
mod types;
pub use types::*;

pub mod cu2qu;
pub mod co2qu;
pub mod qu2cu;
