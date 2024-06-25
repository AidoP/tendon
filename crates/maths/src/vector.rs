use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Implements the common vector functions.
macro_rules! vector_impl {
    // "lhs, rhs, rhs and rhs"
    (@text_list; $lhs:expr) => { $lhs };
    (@text_list; $lhs:expr, $rhs:expr) => { concat!($lhs, " and ", $rhs) };
    (@text_list; $lhs:expr $(,$rhs:expr)*) => { concat!($lhs, ", ", vector_impl!(@text_list; $($rhs),*)) };
    // lhs + rhs + rhs
    (@add; $lhs:expr) => { $lhs };
    (@add; $lhs:expr, $rhs:expr) => { $lhs + $rhs };
    (@add; $lhs:expr $(,$rhs:expr)*) => { $lhs + vector_impl!(@add; $($rhs),*) };
    // convert a metavariable to something else
    (@convert; $in:tt, $out:tt) => { $out };
    // "lhs, rhs, rhs, rhs"
    (@concat_list_convert; $out:literal, $lhs:expr) => { $out };
    (@concat_list_convert; $out:literal, $lhs:expr, $rhs:expr) => { concat!($out, ", ", $out) };
    (@concat_list_convert; $out:literal, $lhs:expr $(,$rhs:expr)*) => { concat!($out $(, ", ", vector_impl!(@convert; $rhs, $out))*) };
    (
        @inner;
        Self = $self:ty,
        SelfValue = $self_value:expr,
        TupleType = $tuple_type:expr,
        TupleValue = $tuple_value:expr,
        TupleComponent = $tuple_component:expr,
        ArrayType = $array_type:expr,
        ArrayValue = $array_value:expr,
        ArrayComponent = $array_component:expr,
        Components = ($($component:ident = $value:literal),*),
        ComponentList = $component_list:expr,
        ValueList = $value_list:expr,
        Count = $count:literal,

        MagnitudeResult = $magnitude_result:literal,
        NormalResult = $normal_result:literal,
        DotResult = $dot_result:literal,
        AddResult = $add_result:literal,
        SubResult = $sub_result:literal,
        MulResult = $mul_result:literal,
        DivResult = $div_result:literal,
    ) => {
        impl $self {
            #[doc = concat!("Creates a ", stringify!($count), "-dimensional vector from ", $component_list, " components.")]
            #[inline]
            pub const fn new($($component: f32),*) -> Self {
                Self { $($component),* }
            }
            #[doc = concat!("Convert a [`", stringify!($self), "`] to an array of `", $array_component, "`.")]
            /// ```
            /// # use ::maths::prelude::*;
            /// assert_eq!(
            #[doc = concat!("    <", $array_type, ">::from(", $self_value, "),")]
            #[doc = concat!("    ", $array_value)]
            /// );
            /// ```
            #[inline]
            pub const fn as_array(self) -> [f32; $count] {
                [$(self.$component),*]
            }
            #[doc = concat!("Convert an array of `", $array_component, "` to a [`", stringify!($self), "`].")]
            /// ```
            /// # use ::maths::prelude::*;
            /// assert_eq!(
            #[doc = concat!("    ", stringify!($self), "::from(", $array_value, "),")]
            #[doc = concat!("    ", $self_value)]
            /// );
            /// ```
            #[inline]
            pub const fn from_array([$($component),*]: [f32; $count]) -> Self {
                Self { $($component),* }
            }
            #[doc = concat!("Convert a [`", stringify!($self), "`] to a tuple of `", $tuple_component, "`.")]
            /// ```
            /// # use ::maths::prelude::*;
            /// assert_eq!(
            #[doc = concat!("    <", $tuple_type, ">::from(", $self_value, "),")]
            #[doc = concat!("    ", $tuple_value)]
            /// );
            /// ```
            #[inline]
            pub const fn as_tuple(self) -> ($(vector_impl!(@convert; $component, f32)),*) {
                ($(self.$component),*)
            }
            #[doc = concat!("Convert a tuple of `", $tuple_component, "` to a [`", stringify!($self), "`].")]
            /// ```
            /// # use ::maths::prelude::*;
            /// assert_eq!(
            #[doc = concat!("    ", stringify!($self), "::from(", $tuple_value, "),")]
            #[doc = concat!("    ", $self_value)]
            /// );
            /// ```
            #[inline]
            pub const fn from_tuple(($($component),*): ($(vector_impl!(@convert; $component, f32)),*)) -> Self {
                Self { $($component),* }
            }
            /// Returns the magnitude of the vector, also known as the length.
            /// ```
            /// # use ::maths::prelude::*;
            /// ::approx::assert_ulps_eq!(
            #[doc = concat!("    ", stringify!($self), "::new(", $value_list, ").magnitude(),")]
            #[doc = concat!("    ", $magnitude_result)]
            /// );
            /// ```
            pub fn magnitude(self) -> f32 {
                (vector_impl!(@add; $(self.$component.powi(2)),*)).sqrt()
            }
            /// Returns the normalised vector, also known as the unit vector.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let normal = ", stringify!($self), "::new(", $value_list, ").normal();")]
            #[doc = concat!("let expected = ", stringify!($self), "::new(", $normal_result, ");")]
            /// ::approx::assert_ulps_eq!(
            ///     normal.as_array().as_slice(),
            ///     expected.as_array().as_slice(),
            /// );
            /// ```
            pub fn normal(self) -> Self {
                let m = self.magnitude();
                Self {
                    $($component: self.$component / m,)*
                }
            }
            /// Returns the dot product of the vector, also known as the scalar product.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let lhs = ", stringify!($self), "::new(", $value_list, ");")]
            #[doc = concat!("let rhs = ", stringify!($self), "::new(", $value_list, ");")]
            /// ::approx::assert_ulps_eq!(
            ///     lhs.dot(rhs),
            #[doc = concat!("    ", $dot_result)]
            /// );
            /// ```
            pub fn dot(self, rhs: Self) -> f32 {
                vector_impl!(@add; $(self.$component * rhs.$component),*)
            }
        }
        impl From<$self> for [f32; $count] {
            #[doc = concat!("See [`", stringify!($self), "::as_array()`].")]
            fn from(value: $self) -> Self {
                value.as_array()
            }
        }
        impl From<[f32; $count]> for $self {
            #[doc = concat!("See [`", stringify!($self), "::from_array()`].")]
            fn from(value: [f32; $count]) -> Self {
                Self::from_array(value)
            }
        }
        impl From<$self> for ($(vector_impl!(@convert; $component, f32)),*) {
            #[doc = concat!("See [`", stringify!($self), "::as_tuple()`].")]
            fn from(value: $self) -> Self {
                value.as_tuple()
            }
        }
        impl From<($(vector_impl!(@convert; $component, f32)),*)> for $self {
            #[doc = concat!("See [`", stringify!($self), "::from_tuple()`].")]
            fn from(value: ($(vector_impl!(@convert; $component, f32)),*)) -> Self {
                Self::from_tuple(value)
            }
        }

        impl Add<f32> for $self {
            type Output = Self;
            /// Adds the scalar value `s` to each component of the vector.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let v = ", stringify!($self), "::new(", $value_list, ") + 1.0;")]
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $add_result, ".as_slice()")]
            /// );
            /// ```
            fn add(self, s: f32) -> Self::Output {
                Self {
                    $($component: self.$component + s,)*
                }
            }
        }
        impl AddAssign<f32> for $self {
            /// Adds the scalar value `s` to each component of the vector.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let mut v = ", stringify!($self), "::new(", $value_list, ");")]
            /// v += 1.0;
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $add_result, ".as_slice()")]
            /// );
            /// ```
            fn add_assign(&mut self, s: f32) {
                $(self.$component += s;)*
            }
        }
        impl Sub<f32> for $self {
            type Output = Self;
            /// Subtracts the scalar value `s` from each component of the vector.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let v = ", stringify!($self), "::new(", $value_list, ") - 1.0;")]
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $sub_result, ".as_slice()")]
            /// );
            /// ```
            fn sub(self, s: f32) -> Self::Output {
                Self {
                    $($component: self.$component - s,)*
                }
            }
        }
        impl SubAssign<f32> for $self {
            /// Subtracts the scalar value `s` from each component of the vector.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let mut v = ", stringify!($self), "::new(", $value_list, ");")]
            /// v -= 1.0;
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $sub_result, ".as_slice()")]
            /// );
            /// ```
            fn sub_assign(&mut self, s: f32) {
                $(self.$component -= s;)*
            }
        }
        impl Mul<f32> for $self {
            type Output = Self;
            /// Multiplies each component of the vector by the scalar value `s`.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let v = ", stringify!($self), "::new(", $value_list, ") * 2.0;")]
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $mul_result, ".as_slice()")]
            /// );
            /// ```
            fn mul(self, s: f32) -> Self::Output {
                Self {
                    $($component: self.$component * s,)*
                }
            }
        }
        impl MulAssign<f32> for $self {
            /// Multiplies each component of the vector by the scalar value `s`.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let mut v = ", stringify!($self), "::new(", $value_list, ");")]
            /// v *= 2.0;
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $mul_result, ".as_slice()")]
            /// );
            /// ```
            fn mul_assign(&mut self, s: f32) {
                $(self.$component *= s;)*
            }
        }
        impl Div<f32> for $self {
            type Output = Self;
            /// Divides each component of the vector by the scalar value `s`.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let v = ", stringify!($self), "::new(", $value_list, ") / 2.0;")]
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $div_result, ".as_slice()")]
            /// );
            /// ```
            fn div(self, s: f32) -> Self::Output {
                Self {
                    $($component: self.$component / s,)*
                }
            }
        }
        impl DivAssign<f32> for $self {
            /// Divides each component of the vector by the scalar value `s`.
            /// ```
            /// # use ::maths::prelude::*;
            #[doc = concat!("let mut v = ", stringify!($self), "::new(", $value_list, ");")]
            /// v /= 2.0;
            /// ::approx::assert_ulps_eq!(
            ///     v.as_array().as_slice(),
            #[doc = concat!("    ", $div_result, ".as_slice()")]
            /// );
            /// ```
            fn div_assign(&mut self, s: f32) {
                $(self.$component /= s;)*
            }
        }
    };
    (
        Self = $self:ident,
        Components = ($($component:ident = $value:literal),*),
        Count = $count:literal,

        MagnitudeResult = $magnitude_result:literal,
        NormalResult = $normal_result:literal,
        DotResult = $dot_result:literal,
        AddResult = $add_result:literal,
        SubResult = $sub_result:literal,
        MulResult = $mul_result:literal,
        DivResult = $div_result:literal,
    ) => {
        vector_impl! {
            @inner;
            Self = $self,
            SelfValue = stringify!($self { $($component: $value),* }),
            TupleType = concat!("(", vector_impl!(@concat_list_convert; "f32", $($component),*), ")"),
            TupleValue = stringify!(($($value),*)),
            TupleComponent = stringify!(($($component),*)),
            ArrayType = stringify!([f32; $count]),
            ArrayValue = stringify!([$($value),*]),
            ArrayComponent = stringify!([$($component),*]),
            Components = ($($component = $value),*),
            ComponentList = vector_impl!(@text_list; $(concat!("`", stringify!($component), "`")),*),
            ValueList = stringify!($($value),*),
            Count = $count,

            MagnitudeResult = $magnitude_result,
            NormalResult = $normal_result,
            DotResult = $dot_result,
            AddResult = $add_result,
            SubResult = $sub_result,
            MulResult = $mul_result,
            DivResult = $div_result,
        }
    };
}

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
vector_impl! {
    Self = Vector2,
    Components = (x = 3.0, y = 4.0),
    Count = 2,

    MagnitudeResult = "5.0",
    NormalResult = "0.6, 0.8",
    DotResult = "25.0",
    AddResult = "[4.0, 5.0]",
    SubResult = "[2.0, 3.0]",
    MulResult = "[6.0, 8.0]",
    DivResult = "[1.5, 2.0]",
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
vector_impl! {
    Self = Vector3,
    Components = (x = 0.0, y = 3.0, z = 4.0),
    Count = 3,

    MagnitudeResult = "5.0",
    NormalResult = "0.0, 0.6, 0.8",
    DotResult = "25.0",
    AddResult = "[1.0, 4.0, 5.0]",
    SubResult = "[-1.0, 2.0, 3.0]",
    MulResult = "[0.0, 6.0, 8.0]",
    DivResult = "[0.0, 1.5, 2.0]",
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
vector_impl! {
    Self = Vector4,
    Components = (x = 4.0, y = 1.0, z = 2.0, w = 2.0),
    Count = 4,

    MagnitudeResult = "5.0",
    NormalResult = "0.8, 0.2, 0.4, 0.4",
    DotResult = "25.0",
    AddResult = "[5.0, 2.0, 3.0, 3.0]",
    SubResult = "[3.0, 0.0, 1.0, 1.0]",
    MulResult = "[8.0, 2.0, 4.0, 4.0]",
    DivResult = "[2.0, 0.5, 1.0, 1.0]",
}
