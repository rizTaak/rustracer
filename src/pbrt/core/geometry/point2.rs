use crate::pbrt;
use crate::pbrt::Component;
use crate::pbrt::HasNaN;
use crate::pbrt::Vector2;
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

#[derive(Debug, Default, Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Component> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        debug_assert!(!x.has_nan());
        debug_assert!(!y.has_nan());
        Self { x: x, y: y }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Component + From<U>, U: Component> From<Vector2<U>> for Point2<T> {
    fn from(item: Vector2<U>) -> Self {
        Self::new(item.x.into(), item.y.into())
    }
}

impl<T: Component + From<U>, U: Component> From<Point2<U>> for Vector2<T> {
    fn from(item: Point2<U>) -> Self {
        Self::new(item.x.into(), item.y.into())
    }
}

impl<T: Component> HasNaN for Point2<T> {
    fn has_nan(&self) -> bool {
        return self.x.has_nan() || self.y.has_nan();
    }
}

impl<T: Component> PartialEq for Point2<T> {
    // todo: with floats nan != nan ?
    fn eq(&self, rhs: &Point2<T>) -> bool {
        return self.x == rhs.x && self.y == rhs.y;
    }
}

impl<T: Component> Add<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        debug_assert!(!self.has_nan());
        return Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Component> AddAssign<Vector2<T>> for Point2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Component> AddAssign for Point2<T> {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Component> Add for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(!self.has_nan());
        return Self::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl<T: Component> Sub<Vector2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        debug_assert!(!self.has_nan());
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Component> Sub for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Component> Neg for Point2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y);
    }
}

impl<T: Component> SubAssign<Vector2<T>> for Point2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl<T: Component> Mul<T> for Point2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        return Self::new(self.x * rhs, self.y * rhs);
    }
}

impl<T: Component> MulAssign<T> for Point2<T> {
    fn mul_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x * rhs, self.y * rhs);
    }
}

impl<T: Component> Div<T> for Point2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        debug_assert_ne!(rhs, T::zero());
        let inv = T::one() / rhs;
        return Self::new(self.x * inv, self.y * inv);
    }
}

impl<T: Component> DivAssign<T> for Point2<T> {
    fn div_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x / rhs, self.y / rhs);
    }
}

impl<T: Component> Index<pbrt::Idx> for Point2<T> {
    type Output = T;

    fn index(&self, idx: pbrt::Idx) -> &Self::Output {
        debug_assert!(0 <= idx && idx <= 1);
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index {} to access vector2 component", idx),
        }
    }
}

// todo: not sure
//impl<T: Component> Eq for Point2<T> {}

// todo: reference or direct value

#[allow(dead_code)]
pub type Point2f = Point2<pbrt::Float>;
#[allow(dead_code)]
pub type Point2i = Point2<i32>;

#[cfg(test)]
mod tests {
    use crate::pbrt;
    use crate::pbrt::HasNaN;
    use crate::pbrt::Point2f;
    use crate::pbrt::Vector2f;

    #[test]
    pub fn test_point2_addassign_vector2() {
        let mut pt = Point2f::new(2.0, 4.0);
        let vec = Vector2f::new(3.0, 5.0);
        pt += vec;
        assert_eq!(pt.x, 5.0);
        assert_eq!(pt.y, 9.0);
    }

    #[test]
    pub fn test_point2_add_vector2() {
        let pt = Point2f::new(2.0, 4.0);
        let vec = Vector2f::new(3.0, 5.0);
        let result = pt + vec;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 9.0);
    }

    #[test]
    pub fn test_point2_vector2() {
        let pt = Point2f::new(2.0, 4.0);
        let vec = Vector2f::from(pt);
        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, 4.0);
    }

    #[test]
    pub fn test_vector2_point2() {
        let vec = Vector2f::new(2.0, 4.0);
        let pt = Point2f::from(vec);
        assert_eq!(pt.x, 2.0);
        assert_eq!(pt.y, 4.0);
    }

    #[test]
    pub fn test_point2_sub_vector() {
        let left = super::Point2f::new(3.0, 6.0);
        let right = Vector2f::new(2.0, 4.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    pub fn test_point2_chain() {
        let left = super::Point2f::new(3.0, 6.0);
        let right = super::Point2f::new(3.0, 6.0);
        let result = ((left + right) / 3.0) * 0.5;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    pub fn test_point2_basic() {
        let left = super::Point2i::new(1, 2);
        let right = super::Point2i::new(3, 4);
        let sum = left + right;
        assert_eq!(sum.x, 4);
        assert_eq!(sum.y, 6);
    }

    #[test]
    pub fn test_point2_idx() {
        let left = super::Point2f::new(2.0, 4.0);
        assert_eq!(left[0], 2.0);
        assert_eq!(left[1], 4.0);
    }

    #[test]
    pub fn test_point2_div_assign_scalar() {
        let mut left = super::Point2f::new(2.0, 4.0);
        left /= 2.0_f32;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_point2_mul_assign_scalar() {
        let mut left = super::Point2f::new(2.0, 3.0);
        left *= 2.0_f32;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_point2_div_scalar() {
        let mut left = super::Point2f::new(2.0, 4.0);
        left = left / 2.0_f32;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_point2_mul_scalar() {
        let mut left = super::Point2f::new(2.0, 3.0);
        left = left * 2.0;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_point2_eq() {
        let left = super::Point2f::new(1.0, 2.0);
        let right = super::Point2f::new(1.0, 2.0);
        assert_eq!(left == right, true);
    }

    #[test]
    pub fn test_point2_default() {
        let vec2f = super::Point2f::default();
        assert_eq!(vec2f.x, 0.0);
        assert_eq!(vec2f.y, 0.0);
    }

    #[test]
    pub fn test_point2_new() {
        let vec2f = super::Point2f::new(1.0, 2.0);
        assert_eq!(vec2f.x, 1.0);
        assert_eq!(vec2f.y, 2.0);
    }

    #[test]
    pub fn test_point2_nan() {
        let vec2f = super::Point2f {
            x: pbrt::Float::NAN,
            y: pbrt::Float::NAN,
        };
        assert_eq!(vec2f.has_nan(), true);
    }

    #[test]
    pub fn test_point2_copy() {
        let vec2f = super::Point2f::new(1.0, 2.0);
        let cp = vec2f;
        assert_eq!(cp.x, vec2f.x);
        assert_eq!(cp.y, vec2f.y);
    }

    #[test]
    pub fn test_point2_add() {
        let left = super::Point2f::new(1.0, 2.0);
        let right = super::Point2f::new(3.0, 4.0);
        let result = left + right;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    pub fn test_point2_add_assign() {
        let mut left = super::Point2f::new(1.0, 2.0);
        let right = super::Point2f::new(3.0, 4.0);
        left += right;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
    }

    #[test]
    pub fn test_point2_sub_assign() {
        let mut left = super::Point2f::new(3.0, 6.0);
        let right = Vector2f::new(2.0, 4.0);
        left -= right;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
    }

    #[test]
    pub fn test_point2_sub() {
        let left = super::Point2f::new(3.0, 6.0);
        let right = super::Point2f::new(2.0, 4.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
    }

    #[test]
    pub fn test_point2_neg() {
        let left = super::Point2f::new(3.0, 6.0);
        let neg = -left;
        assert_eq!(neg.x, -3.0);
        assert_eq!(neg.y, -6.0);
    }
}
