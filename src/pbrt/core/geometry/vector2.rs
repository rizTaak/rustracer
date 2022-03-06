use core::fmt::Debug;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Index;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Neg;
use core::ops::Sub;
use core::ops::SubAssign;

use crate::pbrt::{Float, HasNaN, Int, Scalar};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Scalar> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        debug_assert!(!x.has_nan());
        debug_assert!(!y.has_nan());
        Self { x, y }
    }

    pub fn length_squared(&self) -> Float {
        let squared = self.x * self.x + self.y * self.y;
        squared.to_float()
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }
}

impl<T: Scalar> HasNaN for Vector2<T> {
    fn has_nan(&self) -> bool {
        self.x.has_nan() || self.y.has_nan()
    }
}

impl<T: Scalar> PartialEq for Vector2<T> {
    // todo: with floats nan != nan ?
    fn eq(&self, rhs: &Vector2<T>) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

// todo: not sure
//impl<T: Component> Eq for Vector2<T> {}

// todo: reference or direct value
impl<T: Scalar> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(!self.has_nan());
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Scalar> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Scalar> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(!rhs.has_nan());
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Scalar> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl<T: Scalar> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Scalar> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x * rhs, self.y * rhs);
    }
}

impl<T: Scalar> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        debug_assert_ne!(rhs, T::zero());
        let inv = T::one() / rhs;
        Self::new(self.x * inv, self.y * inv)
    }
}

impl<T: Scalar> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x / rhs, self.y / rhs);
    }
}

impl<T: Scalar> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Scalar> Index<Int> for Vector2<T> {
    type Output = T;

    fn index(&self, idx: Int) -> &Self::Output {
        debug_assert!((0..=1).contains(&idx));
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index {} to access vector2 component", idx),
        }
    }
}

pub type Vector2f = Vector2<Float>;
pub type Vector2i = Vector2<i32>;

#[cfg(test)]
mod tests {
    use crate::pbrt::{Float, HasNaN};

    #[test]
    pub fn tst_vector2_chain() {
        let left = super::Vector2f::new(3.0, 6.0);
        let right = super::Vector2f::new(3.0, 6.0);
        let result = ((left + right) / 3.0) * 0.5;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    pub fn test_vector2_length() {
        let left = super::Vector2f::new(3.0, 4.0);
        let length = left.length();
        assert_eq!(length, 5.0)
    }

    #[test]
    pub fn test_vector2i_basic() {
        let left = super::Vector2i::new(1, 2);
        let right = super::Vector2i::new(3, 4);
        let sum = left + right;
        assert_eq!(sum.x, 4);
        assert_eq!(sum.y, 6);
    }

    #[test]
    pub fn test_vector2_idx() {
        let left = super::Vector2f::new(2.0, 4.0);
        assert_eq!(left[0], 2.0);
        assert_eq!(left[1], 4.0);
    }

    #[test]
    pub fn test_vector2_div_assign_scalar() {
        let mut left = super::Vector2f::new(2.0, 4.0);
        left /= 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_vector2_mul_assign_scalar() {
        let mut left = super::Vector2f::new(2.0, 3.0);
        left *= 2.0 as Float;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_vector2_div_scalar() {
        let mut left = super::Vector2f::new(2.0, 4.0);
        left /= 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_vector2_mul_scalar() {
        let mut left = super::Vector2f::new(2.0, 3.0);
        left *= 2.0;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_vector2_eq() {
        let left = super::Vector2f::new(1.0, 2.0);
        let right = super::Vector2f::new(1.0, 2.0);
        assert!(left == right);
    }

    #[test]
    pub fn test_vector2f_default() {
        let vec2f = super::Vector2f::default();
        assert_eq!(vec2f.x, 0.0);
        assert_eq!(vec2f.y, 0.0);
    }

    #[test]
    pub fn test_vector2f_new() {
        let vec2f = super::Vector2f::new(1.0, 2.0);
        assert_eq!(vec2f.x, 1.0);
        assert_eq!(vec2f.y, 2.0);
    }

    #[test]
    pub fn test_vector2f_nan() {
        let vec2f = super::Vector2f {
            x: Float::NAN,
            y: Float::NAN,
        };
        assert!(vec2f.has_nan());
    }

    #[test]
    pub fn test_vector2f_copy() {
        let vec2f = super::Vector2f::new(1.0, 2.0);
        let cp = vec2f;
        assert_eq!(cp.x, vec2f.x);
        assert_eq!(cp.y, vec2f.y);
    }

    #[test]
    pub fn test_vector2f_add() {
        let left = super::Vector2f::new(1.0, 2.0);
        let right = super::Vector2f::new(3.0, 4.0);
        let result = left + right;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    pub fn test_vector2f_add_assign() {
        let mut left = super::Vector2f::new(1.0, 2.0);
        let right = super::Vector2f::new(3.0, 4.0);
        left += right;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_vector2f_sub_assign() {
        let mut left = super::Vector2f::new(3.0, 6.0);
        let right = super::Vector2f::new(2.0, 4.0);
        left -= right;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_vector2f_sub() {
        let left = super::Vector2f::new(3.0, 6.0);
        let right = super::Vector2f::new(2.0, 4.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    pub fn test_vector2f_neg() {
        let left = super::Vector2f::new(3.0, 6.0);
        let neg = -left;
        assert_eq!(neg.x, -3.0);
        assert_eq!(neg.y, -6.0);
    }
}
