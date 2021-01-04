use std::ops::{Deref, DerefMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// A 2-dimensional vector with f64 components
/// ```rust
/// use tendon::*;
/// let v = Vector2 { x: 0.0, y: 0.0 };
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}
impl Vector2 {
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    /// The length of the vector
    /// ```rust
    /// use tendon::*;
    /// let v = Vector2 { x: 3.0, y: 4.0 };
    /// let dif = v.magnitude() - 5.0f64;
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2))
    }
    /// Normalise the vector such that it points in the same direction but with a magnitude of 1
    /// ```rust
    /// use tendon::*;
    /// let v = Vector2 { x: 3.0, y: 4.0 };
    /// let dif = v.normal().magnitude() - 1.0;
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn normal(self) -> Self {
        let f = 1.0 / self.magnitude();
        Self {
            x: self.x * f,
            y: self.y * f
        }
    }
    /// The dot (scalar) product of self and other.
    /// ```rust
    /// use tendon::*;
    /// let a = Vector2 { x: 3.0, y: 4.0 };
    /// let b = Vector2 { x: -1.0, y: 1.5 };
    /// let dif = a.dot(b) - 3.0;
    /// assert!(dif < 1e-10);
    /// ```
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add<f64> for Vector2 {
    type Output = Self;
    fn add(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar
        }
    }
}
impl AddAssign<f64> for Vector2 {
    fn add_assign(&mut self, scalar: f64) {
        self.x += scalar;
        self.y += scalar;
    }
}
impl Sub<f64> for Vector2 {
    type Output = Self;
    fn sub(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar
        }
    }
}
impl SubAssign<f64> for Vector2 {
    fn sub_assign(&mut self, scalar: f64) {
        self.x -= scalar;
        self.y -= scalar;
    }
}
impl Mul<f64> for Vector2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}
impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}
impl Div<f64> for Vector2 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}
impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, vector: Self) -> Self::Output {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y
        }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, vector: Self) {
        self.x += vector.x;
        self.y += vector.y;
    }
}
impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, vector: Self) -> Self::Output {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y
        }
    }
}
impl SubAssign for Vector2 {
    fn sub_assign(&mut self, vector: Self) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
}

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

/// A 4-dimensional vector with f64 components
/// ```rust
/// use tendon::*;
/// let v = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl Vector4 {
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    /// The length of the vector
    /// ```rust
    /// use tendon::*;
    /// let v = Vector4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 };
    /// let dif = v.magnitude() - 86.0f64.sqrt();
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2))
    }
    /// Normalise the vector such that it points in the same direction but with a magnitude of 1
    /// ```rust
    /// use tendon::*;
    /// let v = Vector4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 };
    /// let dif = v.normal().magnitude() - 1.0;
    /// assert!(dif.abs() < 1e-10);
    /// ```
    pub fn normal(self) -> Self {
        let f = 1.0 / self.magnitude();
        Self {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f
        }
    }
    /// The dot (scalar) product of self and other.
    /// ```rust
    /// use tendon::*;
    /// let a = Vector4 { x: 3.0, y: 4.0, z: 5.0, w: 6.0 };
    /// let b = Vector4 { x: -1.0, y: 1.5, z: 0.5, w: -1.0 / 3.0 };
    /// let dif = a.dot(b) - 3.5;
    /// assert!(dif < 1e-10);
    /// ```
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl Add<f64> for Vector4 {
    type Output = Self;
    fn add(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
            w: self.w + scalar
        }
    }
}
impl AddAssign<f64> for Vector4 {
    fn add_assign(&mut self, scalar: f64) {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
        self.w += scalar;
    }
}
impl Sub<f64> for Vector4 {
    type Output = Self;
    fn sub(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
            w: self.w - scalar
        }
    }
}
impl SubAssign<f64> for Vector4 {
    fn sub_assign(&mut self, scalar: f64) {
        self.x -= scalar;
        self.y -= scalar;
        self.z -= scalar;
        self.w -= scalar;
    }
}
impl Mul<f64> for Vector4 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar
        }
    }
}
impl MulAssign<f64> for Vector4 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}
impl Div<f64> for Vector4 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar
        }
    }
}
impl DivAssign<f64> for Vector4 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

impl Add for Vector4 {
    type Output = Self;
    fn add(self, vector: Self) -> Self::Output {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
            w: self.w + vector.w
        }
    }
}
impl AddAssign for Vector4 {
    fn add_assign(&mut self, vector: Self) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
        self.w += vector.w;
    }
}
impl Sub for Vector4 {
    type Output = Self;
    fn sub(self, vector: Self) -> Self::Output {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
            w: self.w - vector.w
        }
    }
}
impl SubAssign for Vector4 {
    fn sub_assign(&mut self, vector: Self) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
        self.w -= vector.w;
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix4(pub [[f64; 4]; 4]);
impl Matrix4 {
    pub fn copy(&self) -> [[f64; 4]; 4] {
        **self
    }
}
impl Deref for Matrix4 {
    type Target = [[f64; 4]; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Add<f64> for Matrix4 {
    type Output = Self;
    fn add(mut self, scalar: f64) -> Self {
        self.iter_mut().flatten().for_each(|v| *v += scalar);
        self
    }
}
impl AddAssign<f64> for Matrix4 {
    fn add_assign(&mut self, scalar: f64) {
        self.iter_mut().flatten().for_each(|v| *v += scalar);
    }
}
impl Sub<f64> for Matrix4 {
    type Output = Self;
    fn sub(mut self, scalar: f64) -> Self {
        self.iter_mut().flatten().for_each(|v| *v -= scalar);
        self
    }
}
impl SubAssign<f64> for Matrix4 {
    fn sub_assign(&mut self, scalar: f64) {
        self.iter_mut().flatten().for_each(|v| *v -= scalar);
    }
}
impl Mul<f64> for Matrix4 {
    type Output = Self;
    fn mul(mut self, scalar: f64) -> Self {
        self.iter_mut().flatten().for_each(|v| *v *= scalar);
        self
    }
}
impl MulAssign<f64> for Matrix4 {
    fn mul_assign(&mut self, scalar: f64) {
        self.iter_mut().flatten().for_each(|v| *v *= scalar);
    }
}
impl Div<f64> for Matrix4 {
    type Output = Self;
    fn div(mut self, scalar: f64) -> Self {
        self.iter_mut().flatten().for_each(|v| *v /= scalar);
        self
    }
}
impl DivAssign<f64> for Matrix4 {
    fn div_assign(&mut self, scalar: f64) {
        self.iter_mut().flatten().for_each(|v| *v /= scalar);
    }
}

/// ```rust
/// use tendon::*;
/// let m = Matrix4([[1.0; 4]; 4]);
/// let r = m + m;
/// let dif = r.iter().flatten().sum::<f64>() - 32.0;
/// assert!(dif < 1e-10);
/// ```
impl Add for Matrix4 {
    type Output = Self;
    fn add(mut self, matrix: Self) -> Self {
        self.iter_mut().zip(matrix.iter()).for_each(|(left, right)| left.iter_mut().zip(right.iter()).for_each(|(left, right)| *left += right));
        self
    }
}
impl AddAssign for Matrix4 {
    fn add_assign(&mut self, matrix: Self) {
        self.iter_mut().zip(matrix.iter()).for_each(|(left, right)| left.iter_mut().zip(right.iter()).for_each(|(left, right)| *left += right))
    }
}
impl Sub for Matrix4 {
    type Output = Self;
    fn sub(mut self, matrix: Self) -> Self {
        self.iter_mut().zip(matrix.iter()).for_each(|(left, right)| left.iter_mut().zip(right.iter()).for_each(|(left, right)| *left -= right));
        self
    }
}
impl SubAssign for Matrix4 {
    fn sub_assign(&mut self, matrix: Self) {
        self.iter_mut().zip(matrix.iter()).for_each(|(left, right)| left.iter_mut().zip(right.iter()).for_each(|(left, right)| *left -= right))
    }
}
/// ```rust
/// use tendon::*;
/// let a = Matrix4([
///     [1.0, 2.0, 3.0, 4.0],
///     [5.0, 6.0, 7.0, 8.0],
///     [9.0, 10.0, 11.0, 12.0],
///     [13.0, 14.0, 15.0, 16.0]
/// ]);
/// let b = Matrix4([
///     [6.0, 3.5, 2.7, 0.0],
///     [0.5, 8.0, -9.81, -2.0],
///     [1.0 / 3.0, 100.0, 0.001, -8.0],
///     [42.0, 7.11, 5.0, -4.3]
/// ]);
/// let expected = Matrix4([
///     [176.0, 347.94, 3.083, -45.2],
///     [371.333, 822.38, -5.353, -102.4],
///     [566.667, 1296.82, -13.789, -159.6],
///     [762.0, 1771.26, -22.225, -216.8]
/// ]);
/// const MAX_ERROR: f64 = 5e-4 + 1e-10;
/// let result = a * b;
/// result.iter().flatten().zip(expected.iter().flatten()).for_each(|(r,e)| assert!((r - e).abs() < MAX_ERROR));
/// ```
impl Mul for Matrix4 {
    type Output = Self;
    fn mul(self, matrix: Self) -> Self {
        macro_rules! rc {
            ($row:expr, $col:expr) => {
                self[$row][0] * matrix[0][$col] +
                self[$row][1] * matrix[1][$col] +
                self[$row][2] * matrix[2][$col] +
                self[$row][3] * matrix[3][$col]
            };
        }
        Self([
            [rc!(0, 0), rc!(0, 1), rc!(0, 2), rc!(0, 3)],
            [rc!(1, 0), rc!(1, 1), rc!(1, 2), rc!(1, 3)],
            [rc!(2, 0), rc!(2, 1), rc!(2, 2), rc!(2, 3)],
            [rc!(3, 0), rc!(3, 1), rc!(3, 2), rc!(3, 3)]
        ])
    }
}