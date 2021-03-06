use core::fmt::Debug;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Neg;
use core::ops::Sub;
use core::ops::SubAssign;
use std::ops::DivAssign;

#[cfg(PBRT_FLOAT_AS_DOUBLE)]
pub type Float = f64;

#[cfg(not(PBRT_FLOAT_AS_DOUBLE))]
pub type Float = f32;

pub type Int = i32;

pub trait HasNaN {
    fn has_nan(&self) -> bool;
}

impl HasNaN for Float {
    #[inline(always)]
    fn has_nan(&self) -> bool {
        self.is_nan()
    }
}

impl HasNaN for Int {
    #[inline(always)]
    fn has_nan(&self) -> bool {
        false
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for Float {
    #[inline(always)]
    fn zero() -> Self {
        0 as Float
    }
}

impl Zero for Int {
    #[inline(always)]
    fn zero() -> Self {
        0
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for Float {
    #[inline(always)]
    fn one() -> Self {
        1 as Float
    }
}

impl One for Int {
    #[inline(always)]
    fn one() -> Self {
        1
    }
}

pub trait IntoFloat {
    fn to_float(&self) -> Float;
}

impl IntoFloat for Int {
    #[inline(always)]
    fn to_float(&self) -> Float {
        *self as Float
    }
}

impl IntoFloat for Float {
    #[inline(always)]
    fn to_float(&self) -> Float {
        *self
    }
}

pub trait FromFloat {
    fn from_float(val: Float) -> Self;
}

impl FromFloat for Float {
    #[inline(always)]
    fn from_float(val: Float) -> Self {
        val as Self
    }
}

impl FromFloat for Int {
    #[inline(always)]
    fn from_float(val: Float) -> Self {
        val as Self
    }
}

pub trait Min: Sized + PartialOrd {
    fn min_value() -> Self;

    #[inline(always)]
    fn min(left: Self, right: Self) -> Self {
        if left < right {
            left
        } else {
            right
        }
    }
}

impl Min for Int {
    #[inline(always)]
    fn min_value() -> Self {
        Int::MIN
    }
}

impl Min for Float {
    #[inline(always)]
    fn min_value() -> Self {
        Float::MIN
    }
}

pub trait Max: Sized + PartialOrd {
    fn max_value() -> Self;

    #[inline(always)]
    fn max(left: Self, right: Self) -> Self {
        if left > right {
            left
        } else {
            right
        }
    }
}

impl Max for Int {
    #[inline(always)]
    fn max_value() -> Self {
        Int::MAX
    }
}

impl Max for Float {
    #[inline(always)]
    fn max_value() -> Self {
        Float::MAX
    }
}

pub trait Scalar:
    Sized
    + Default
    + Clone
    + Copy
    + HasNaN
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Debug
    + PartialEq
    + IntoFloat
    + FromFloat
    + Neg<Output = Self>
    + Min
    + Max
    + PartialOrd
{
}

impl Scalar for Int {}

impl Scalar for Float {}
