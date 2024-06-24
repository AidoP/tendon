/// 2-dimensional vector.
/// ```
/// # use ::maths::prelude::*;
/// let pos = Vector2 { x: 1.0, y: 2.0 };
/// assert_eq!(pos.x, 1.0);
/// assert_eq!(pos.y, 2.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

/// 3-dimensional vector.
/// ```
/// # use ::maths::prelude::*;
/// let pos = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
/// assert_eq!(pos.x, 1.0);
/// assert_eq!(pos.y, 2.0);
/// assert_eq!(pos.z, 3.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 4-dimensional vector.
/// ```
/// # use ::maths::prelude::*;
/// let pos = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
/// assert_eq!(pos.x, 1.0);
/// assert_eq!(pos.y, 2.0);
/// assert_eq!(pos.z, 3.0);
/// assert_eq!(pos.w, 4.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
