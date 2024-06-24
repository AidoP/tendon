//! # Mathematics Primitives
//! Base primitives for vectors, matrices and more.

mod vector2;
pub use vector2::Vector2;
mod vector3;
pub use vector3::Vector3;
mod vector4;
pub use vector4::Vector4;

pub mod prelude {
    pub use crate::{Vector2, Vector3, Vector4};
}
