use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl Vector3 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    /// Convert a [`Vector3`] to an array of `[x, y, z]`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     <[f32; 3]>::from(Vector3 { x: 1.0, y: 2.0, z: 3.0 }),
    ///     [1.0, 2.0, 3.0]
    /// );
    /// ```
    #[inline]
    pub const fn as_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    /// Convert an array of `[x, y, z]` to a [`Vector3`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     Vector3::from([1.0, 2.0, 3.0]),
    ///     Vector3 { x: 1.0, y: 2.0, z: 3.0 }
    /// );
    /// ```
    #[inline]
    pub const fn from_array([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
    /// Convert a [`Vector3`] to a tuple of `(x, y, z)`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     <(f32, f32, f32)>::from(Vector3 { x: 1.0, y: 2.0, z: 3.0 }),
    ///     (1.0, 2.0, 3.0)
    /// );
    /// ```
    #[inline]
    pub const fn as_tuple(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
    /// Convert a tuple of `(x, y, z)` to a [`Vector3`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     Vector3::from((1.0, 2.0, 3.0)),
    ///     Vector3 { x: 1.0, y: 2.0, z: 3.0 }
    /// );
    /// ```
    #[inline]
    pub const fn from_tuple((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
    /// Returns the magnitude of the vector, also known as the length.
    /// ```
    /// # use ::maths::prelude::*;
    /// ::approx::assert_ulps_eq!(
    ///     Vector3::new(0.0, 3.0, 4.0).magnitude(),
    ///     5.0
    /// );
    /// ```
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    /// Returns the normalised vector, also known as the unit vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let normal = Vector3::new(0.0, 3.0, 4.0).normal();
    /// let expected = Vector3::new(0.0, 0.6, 0.8);
    /// ::approx::assert_ulps_eq!(
    ///     normal.as_array().as_slice(),
    ///     normal.as_array().as_slice()
    /// );
    /// ```
    pub fn normal(self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
    /// Returns the dot product of the vector, also known as the scalar product.
    /// ```
    /// # use ::maths::prelude::*;
    /// let lhs = Vector3::new(3.0, 4.0, 5.0);
    /// let rhs = Vector3::new(3.0, 4.0, 5.0);
    /// ::approx::assert_ulps_eq!(
    ///     lhs.dot(rhs),
    ///     50.0
    /// );
    /// ```
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl From<Vector3> for [f32; 3] {
    /// See [`Vector3::as_array()`].
    fn from(value: Vector3) -> Self {
        value.as_array()
    }
}
impl From<[f32; 3]> for Vector3 {
    /// See [`Vector3::from_array()`].
    fn from(value: [f32; 3]) -> Self {
        Self::from_array(value)
    }
}
impl From<Vector3> for (f32, f32, f32) {
    /// See [`Vector3::as_tuple()`].
    fn from(value: Vector3) -> Self {
        value.as_tuple()
    }
}
impl From<(f32, f32, f32)> for Vector3 {
    /// See [`Vector3::from_tuple()`].
    fn from(value: (f32, f32, f32)) -> Self {
        Self::from_tuple(value)
    }
}

impl Add<f32> for Vector3 {
    type Output = Self;
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector3::new(0.0, 1.0, 2.0) + 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0, 3.0].as_slice()
    /// );
    /// ```
    fn add(self, s: f32) -> Self::Output {
        Self {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
        }
    }
}
impl AddAssign<f32> for Vector3 {
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector3::new(0.0, 1.0, 2.0);
    /// v += 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0, 3.0].as_slice()
    /// );
    /// ```
    fn add_assign(&mut self, s: f32) {
        self.x += s;
        self.y += s;
        self.z += s;
    }
}
impl Sub<f32> for Vector3 {
    type Output = Self;
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector3::new(0.0, 1.0, 2.0) - 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0, 1.0].as_slice()
    /// );
    /// ```
    fn sub(self, s: f32) -> Self::Output {
        Self {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
        }
    }
}
impl SubAssign<f32> for Vector3 {
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector3::new(0.0, 1.0, 2.0);
    /// v -= 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0, 1.0].as_slice()
    /// );
    /// ```
    fn sub_assign(&mut self, s: f32) {
        self.x -= s;
        self.y -= s;
        self.z -= s;
    }
}
impl Mul<f32> for Vector3 {
    type Output = Self;
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector3::new(1.0, 2.0, 3.0) * 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0, 6.0].as_slice()
    /// );
    /// ```
    fn mul(self, s: f32) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}
impl MulAssign<f32> for Vector3 {
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector3::new(1.0, 2.0, 3.0);
    /// v *= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0, 6.0].as_slice()
    /// );
    /// ```
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}
impl Div<f32> for Vector3 {
    type Output = Self;
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector3::new(1.0, 2.0, 3.0) / 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0, 1.5].as_slice()
    /// );
    /// ```
    fn div(self, s: f32) -> Self::Output {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}
impl DivAssign<f32> for Vector3 {
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector3::new(1.0, 2.0, 3.0);
    /// v /= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0, 1.5].as_slice()
    /// );
    /// ```
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}
