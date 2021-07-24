#[cfg(PBRT_FLOAT_AS_DOUBLE)]
pub type Float = f64;

use core::fmt::Debug;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use std::ops::DivAssign;

#[cfg(not(PBRT_FLOAT_AS_DOUBLE))]
pub type Float = f32;

pub trait HasNaN {
    fn has_nan(&self) -> bool;
}

impl HasNaN for i32 {
    fn has_nan(&self) -> bool {
        false
    }
}

impl HasNaN for i64 {
    fn has_nan(&self) -> bool {
        false
    }
}

impl HasNaN for f32 {
    fn has_nan(&self) -> bool {
        self.is_nan()
    }
}

impl HasNaN for f64 {
    fn has_nan(&self) -> bool {
        self.is_nan()
    }
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self {
        0_i32
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0_i64
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0_f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0_f64
    }
}

impl One for i32 {
    fn one() -> Self {
        1_i32
    }
}

impl One for i64 {
    fn one() -> Self {
        1_i64
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0_f32
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0_f64
    }
}

pub trait Component:
    Sized
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
    + Copy
{
}

impl Component for f32 {}

impl Component for i32 {}

impl Component for f64 {}

impl Component for i64 {}
