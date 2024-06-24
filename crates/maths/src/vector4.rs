use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl Vector4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    /// Convert a [`Vector4`] to an array of `[x, y, z, w]`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     <[f32; 4]>::from(Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }),
    ///     [1.0, 2.0, 3.0, 4.0]
    /// );
    /// ```
    #[inline]
    pub const fn as_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
    /// Convert an array of `[x, y, z, w]` to a [`Vector4`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     Vector4::from([1.0, 2.0, 3.0, 4.0]),
    ///     Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
    /// );
    /// ```
    #[inline]
    pub const fn from_array([x, y, z, w]: [f32; 4]) -> Self {
        Self { x, y, z, w }
    }
    /// Convert a [`Vector4`] to a tuple of `(x, y, z, w)`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     <(f32, f32, f32, f32)>::from(Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }),
    ///     (1.0, 2.0, 3.0, 4.0)
    /// );
    /// ```
    #[inline]
    pub const fn as_tuple(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
    /// Convert a tuple of `(x, y, z, w)` to a [`Vector4`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(
    ///     Vector4::from((1.0, 2.0, 3.0, 4.0)),
    ///     Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
    /// );
    /// ```
    #[inline]
    pub const fn from_tuple((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
    /// Returns the magnitude of the vector, also known as the length.
    /// ```
    /// # use ::maths::prelude::*;
    /// ::approx::assert_ulps_eq!(
    ///     Vector4::new(4.0, 1.0, 2.0, 2.0).magnitude(),
    ///     5.0
    /// );
    /// ```
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
    /// Returns the normalised vector, also known as the unit vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let normal = Vector4::new(4.0, 1.0, 2.0, 2.0).normal();
    /// let expected = Vector4::new(0.8, 0.2, 0.4, 0.4);
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
            w: self.w / m,
        }
    }
    /// Returns the dot product of the vector, also known as the scalar product.
    /// ```
    /// # use ::maths::prelude::*;
    /// let lhs = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let rhs = Vector4::new(4.0, 3.0, 2.0, 1.0);
    /// ::approx::assert_ulps_eq!(
    ///     lhs.dot(rhs),
    ///     20.0
    /// );
    /// ```
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}
impl From<Vector4> for [f32; 4] {
    /// See [`Vector4::as_array()`].
    fn from(value: Vector4) -> Self {
        value.as_array()
    }
}
impl From<[f32; 4]> for Vector4 {
    /// See [`Vector4::from_array()`].
    fn from(value: [f32; 4]) -> Self {
        Self::from_array(value)
    }
}
impl From<Vector4> for (f32, f32, f32, f32) {
    /// See [`Vector4::as_tuple()`].
    fn from(value: Vector4) -> Self {
        value.as_tuple()
    }
}
impl From<(f32, f32, f32, f32)> for Vector4 {
    /// See [`Vector4::from_tuple()`].
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self::from_tuple(value)
    }
}

impl Add<f32> for Vector4 {
    type Output = Self;
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector4::new(0.0, 1.0, 2.0, 3.0) + 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0, 3.0, 4.0].as_slice()
    /// );
    /// ```
    fn add(self, s: f32) -> Self::Output {
        Self {
            x: self.x + s,
            y: self.y + s,
            z: self.z + s,
            w: self.w + s,
        }
    }
}
impl AddAssign<f32> for Vector4 {
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector4::new(0.0, 1.0, 2.0, 3.0);
    /// v += 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0, 3.0, 4.0].as_slice()
    /// );
    /// ```
    fn add_assign(&mut self, s: f32) {
        self.x += s;
        self.y += s;
        self.z += s;
        self.w += s;
    }
}
impl Sub<f32> for Vector4 {
    type Output = Self;
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector4::new(0.0, 1.0, 2.0, 3.0) - 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0, 1.0, 2.0].as_slice()
    /// );
    /// ```
    fn sub(self, s: f32) -> Self::Output {
        Self {
            x: self.x - s,
            y: self.y - s,
            z: self.z - s,
            w: self.w - s,
        }
    }
}
impl SubAssign<f32> for Vector4 {
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector4::new(0.0, 1.0, 2.0, 3.0);
    /// v -= 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0, 1.0, 2.0].as_slice()
    /// );
    /// ```
    fn sub_assign(&mut self, s: f32) {
        self.x -= s;
        self.y -= s;
        self.z -= s;
        self.w -= s;
    }
}
impl Mul<f32> for Vector4 {
    type Output = Self;
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector4::new(1.0, 2.0, 3.0, 4.0) * 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0, 6.0, 8.0].as_slice()
    /// );
    /// ```
    fn mul(self, s: f32) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}
impl MulAssign<f32> for Vector4 {
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// v *= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0, 6.0, 8.0].as_slice()
    /// );
    /// ```
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }
}
impl Div<f32> for Vector4 {
    type Output = Self;
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector4::new(1.0, 2.0, 3.0, 4.0) / 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0, 1.5, 2.0].as_slice()
    /// );
    /// ```
    fn div(self, s: f32) -> Self::Output {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
            w: self.w / s,
        }
    }
}
impl DivAssign<f32> for Vector4 {
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// v /= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0, 1.5, 2.0].as_slice()
    /// );
    /// ```
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
        self.z /= s;
        self.w /= s;
    }
}
