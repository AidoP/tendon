use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// A 3-dimensional vector with f64 components
/// ```rust
/// use tendon::*;
/// let v = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl Vector3 {
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    /// The length of the vector
    /// ```rust
    /// use tendon::*;
    /// let v = Vector3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let dif = v.magnitude() - 50.0f64.sqrt();
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
    /// Normalise the vector such that it points in the same direction but with a magnitude of 1
    /// ```rust
    /// use tendon::*;
    /// let v = Vector3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let dif = v.normal().magnitude() - 1.0;
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn normal(self) -> Self {
        let f = 1.0 / self.magnitude();
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f
        }
    }
    /// The dot (scalar) product of self and other.
    /// ```rust
    /// use tendon::*;
    /// let a = Vector3 { x: 3.0, y: 4.0, z: 5.0 };
    /// let b = Vector3 { x: -1.0, y: 1.5, z: 0.5 };
    /// let dif = a.dot(b) - 5.5;
    /// assert!(dif < 1e-10);
    /// ```
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// The cross (vector) product of self and other.
    /// ```rust
    /// use tendon::*;
    /// let x = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    /// let y = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    /// let dif = x.cross(y) - Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    /// const TINY: f64 = 1e-10;
    /// assert!(dif.x.abs() < TINY && dif.y.abs() < TINY && dif.z.abs() < TINY)
    /// ```
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z:self.x * other.y - self.y * other.x
        }
    }
}

impl Add<f64> for Vector3 {
    type Output = Self;
    fn add(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar
        }
    }
}
impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, scalar: f64) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
    }
}
impl Sub<f64> for Vector3 {
    type Output = Self;
    fn sub(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar
        }
    }
}
impl SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, scalar: f64) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
    }
}
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}
impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}
impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}
impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, vector: Self) -> Self::Output {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z
        }
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, vector: Self) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
    }
}
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, vector: Self) -> Self::Output {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z
        }
    }
}
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, vector: Self) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }
}

pub struct Matrix4([[f64; 4]; 4]);