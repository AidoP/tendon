use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

impl Vector2 {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Convert a [`Vector2`] to an array of `[x, y]`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(<[f32; 2]>::from(Vector2 { x: 1.0, y: 2.0 }), [1.0, 2.0]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
    /// Convert an array of `[x, y]` to a [`Vector2`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(Vector2::from([1.0, 2.0]), Vector2 { x: 1.0, y: 2.0 });
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_array([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
    /// Convert a [`Vector2`] to a tuple of `(x, y)`.
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(<(f32, f32)>::from(Vector2 { x: 1.0, y: 2.0 }), (1.0, 2.0));
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }
    /// Convert a tuple of `(x, y)` to a [`Vector2`].
    /// ```
    /// # use ::maths::prelude::*;
    /// assert_eq!(Vector2::from((1.0, 2.0)), Vector2 { x: 1.0, y: 2.0 });
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_tuple((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
    /// Returns the magnitude of the vector, also known as the length.
    /// ```
    /// # use ::maths::prelude::*;
    /// ::approx::assert_ulps_eq!(
    ///     Vector2::new(3.0, 4.0).magnitude(),
    ///     5.0
    /// );
    /// ```
    #[must_use]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    /// Returns the normalised vector, also known as the unit vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector2::new(3.0, 4.0).normal();
    /// let e = Vector2::new(0.6, 0.8);
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     e.as_array().as_slice()
    /// );
    /// ```
    #[must_use]
    pub fn normal(self) -> Self {
        let m = self.magnitude();
        Self {
            x: self.x / m,
            y: self.y / m,
        }
    }
    /// Returns the dot product of the vector, also known as the scalar product.
    /// ```
    /// # use ::maths::prelude::*;
    /// let lhs = Vector2::new(3.0, 4.0);
    /// let rhs = Vector2::new(-1.0, 1.5);
    /// ::approx::assert_ulps_eq!(
    ///     lhs.dot(rhs),
    ///     3.0
    /// );
    /// ```
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl From<Vector2> for [f32; 2] {
    /// See [`Vector2::as_array()`].
    fn from(value: Vector2) -> Self {
        value.as_array()
    }
}
impl From<[f32; 2]> for Vector2 {
    /// See [`Vector2::from_array()`].
    fn from(value: [f32; 2]) -> Self {
        Self::from_array(value)
    }
}
impl From<Vector2> for (f32, f32) {
    /// See [`Vector2::as_tuple()`].
    fn from(value: Vector2) -> Self {
        value.as_tuple()
    }
}
impl From<(f32, f32)> for Vector2 {
    /// See [`Vector2::from_tuple()`].
    fn from(value: (f32, f32)) -> Self {
        Self::from_tuple(value)
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector2::new(0.0, 1.0) + 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0].as_slice()
    /// );
    /// ```
    fn add(self, s: f32) -> Self::Output {
        Self {
            x: self.x + s,
            y: self.y + s,
        }
    }
}
impl AddAssign<f32> for Vector2 {
    /// Adds the scalar value `s` to each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector2::new(0.0, 1.0);
    /// v += 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [1.0, 2.0].as_slice()
    /// );
    /// ```
    fn add_assign(&mut self, s: f32) {
        self.x += s;
        self.y += s;
    }
}
impl Sub<f32> for Vector2 {
    type Output = Self;
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector2::new(0.0, 1.0) - 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0].as_slice()
    /// );
    /// ```
    fn sub(self, s: f32) -> Self::Output {
        Self {
            x: self.x - s,
            y: self.y - s,
        }
    }
}
impl SubAssign<f32> for Vector2 {
    /// Subtracts the scalar value `s` from each component of the vector.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector2::new(0.0, 1.0);
    /// v -= 1.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [-1.0, 0.0].as_slice()
    /// );
    /// ```
    fn sub_assign(&mut self, s: f32) {
        self.x -= s;
        self.y -= s;
    }
}
impl Mul<f32> for Vector2 {
    type Output = Self;
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector2::new(1.0, 2.0) * 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0].as_slice()
    /// );
    /// ```
    fn mul(self, s: f32) -> Self::Output {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
impl MulAssign<f32> for Vector2 {
    /// Multiplies each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector2::new(1.0, 2.0);
    /// v *= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [2.0, 4.0].as_slice()
    /// );
    /// ```
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
    }
}
impl Div<f32> for Vector2 {
    type Output = Self;
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let v = Vector2::new(1.0, 2.0) / 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0].as_slice()
    /// );
    /// ```
    fn div(self, s: f32) -> Self::Output {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}
impl DivAssign<f32> for Vector2 {
    /// Divides each component of the vector by the scalar value `s`.
    /// ```
    /// # use ::maths::prelude::*;
    /// let mut v = Vector2::new(1.0, 2.0);
    /// v /= 2.0;
    /// ::approx::assert_ulps_eq!(
    ///     v.as_array().as_slice(),
    ///     [0.5, 1.0].as_slice()
    /// );
    /// ```
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
    }
}
