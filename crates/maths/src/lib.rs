//! # Mathematics Primitives
//! Base primitives for vectors, matrices and more.

mod vector;
pub use vector::{Vector2, Vector3, Vector4};

pub mod prelude {
    pub use crate::{Vector2, Vector3, Vector4};
}
