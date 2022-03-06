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

use crate::pbrt::{Float, HasNaN, Int, One, Scalar, Vector3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(!x.has_nan());
        debug_assert!(!y.has_nan());
        debug_assert!(!z.has_nan());
        Self { x, y, z }
    }

    // todo: remove this
    pub fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn distance(left: &Point3<T>, right: &Point3<T>) -> Float {
        (left - right).length()
    }

    fn distance_squared(left: &Point3<T>, right: &Point3<T>) -> Float {
        (left - right).length_squared()
    }

    fn lerp(t: Float, p0: &Point3<T>, p1: &Point3<T>) -> Point3<T> {
        p0 * (Float::one() - t) + p1 * t
    }
}

impl<T: Scalar + From<U>, U: Scalar> From<Vector3<U>> for Point3<T> {
    fn from(item: Vector3<U>) -> Self {
        Self::new(item.x.into(), item.y.into(), item.z.into())
    }
}

impl<T: Scalar + From<U>, U: Scalar> From<Point3<U>> for Vector3<T> {
    fn from(item: Point3<U>) -> Self {
        Self::new(item.x.into(), item.y.into(), item.z.into())
    }
}

impl<T: Scalar> HasNaN for Point3<T> {
    fn has_nan(&self) -> bool {
        self.x.has_nan() || self.y.has_nan() && self.z.has_nan()
    }
}

impl<T: Scalar> PartialEq for Point3<T> {
    // todo: with floats nan != nan ?
    fn eq(&self, rhs: &Point3<T>) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl<T: Scalar> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        debug_assert!(!self.has_nan());
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Scalar> AddAssign<Vector3<T>> for Point3<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl<T: Scalar> AddAssign for Point3<T> {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl<T: Scalar> Add for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(!self.has_nan());
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Scalar> Sub<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        debug_assert!(!self.has_nan());
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Scalar> Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Scalar> Sub for &Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Scalar> Neg for Point3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Scalar> SubAssign<Vector3<T>> for Point3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl<T: Scalar> Mul<T> for Point3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Scalar> Mul<Float> for &Point3<T> {
    type Output = Point3<T>;

    fn mul(self, rhs: Float) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        Self::Output::new(
            T::from_float(self.x.to_float() * rhs),
            T::from_float(self.y.to_float() * rhs),
            T::from_float(self.z.to_float() * rhs),
        )
    }
}

impl<T: Scalar> MulAssign<T> for Point3<T> {
    fn mul_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl<T: Scalar> Div<T> for Point3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        debug_assert_ne!(rhs, T::zero());
        let inv = T::one() / rhs;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl<T: Scalar> DivAssign<T> for Point3<T> {
    fn div_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl<T: Scalar> Index<Int> for Point3<T> {
    type Output = T;

    fn index(&self, idx: Int) -> &Self::Output {
        debug_assert!((0..=2).contains(&idx));
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index {} is used to access Point3 scalar", idx),
        }
    }
}

// todo: not sure
//impl<T: Scalar> Eq for Point3<T> {}

// todo: reference or direct value

pub type Point3f = Point3<Float>;
pub type Point2i = Point3<i32>;

#[cfg(test)]
mod tests {
    use crate::pbrt::{Float, HasNaN, Vector3f};

    #[test]
    #[should_panic]
    pub fn test_point3_idx_panic() {
        let pt = super::Point3f::new(2.0, 4.0, 6.0);
        let _value = pt[3];
    }

    #[test]
    pub fn test_point3_distance() {
        let start = super::Point3f::new(2.0, 4.0, 8.0);
        let end = super::Point3f::new(2.0, 4.0, 0.0);
        let dist = super::Point3f::distance(&start, &end);
        assert_eq!(dist, 8.0);
    }

    #[test]
    pub fn test_point3_distance_squared() {
        let start = super::Point3f::new(2.0, 4.0, 8.0);
        let end = super::Point3f::new(2.0, 4.0, 0.0);
        let dist = super::Point3f::distance_squared(&start, &end);
        assert_eq!(dist, 64.0);
    }

    #[test]
    pub fn test_point3_scalar_mul() {
        let start = super::Point3f::new(2.0, 4.0, 8.0);
        let result = start * 2.0;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 8.0);
        assert_eq!(result.z, 16.0);
    }

    #[test]
    pub fn test_point3_addref_vector2() {
        let mut pt = super::Point3f::new(2.0, 4.0, 8.0);
        let vec = Vector3f::new(3.0, 5.0, 7.0);
        pt += vec;
        assert_eq!(pt.x, 5.0);
        assert_eq!(pt.y, 9.0);
        assert_eq!(pt.z, 15.0);
    }

    #[test]
    pub fn test_point3_addassign_vector2() {
        let mut pt = super::Point3f::new(2.0, 4.0, 8.0);
        let vec = Vector3f::new(3.0, 5.0, 7.0);
        pt += vec;
        assert_eq!(pt.x, 5.0);
        assert_eq!(pt.y, 9.0);
        assert_eq!(pt.z, 15.0);
    }

    #[test]
    pub fn test_point3_add_vector2() {
        let pt = super::Point3f::new(2.0, 4.0, 8.0);
        let vec = Vector3f::new(3.0, 5.0, 7.0);
        let result = pt + vec;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 9.0);
        assert_eq!(result.z, 15.0);
    }

    #[test]
    pub fn test_point3_vector2() {
        let pt = super::Point3f::new(2.0, 4.0, 8.0);
        let vec = Vector3f::from(pt);
        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, 4.0);
        assert_eq!(vec.z, 8.0);
    }

    #[test]
    pub fn test_vector2_point2() {
        let vec = Vector3f::new(2.0, 4.0, 8.0);
        let pt = super::Point3f::from(vec);
        assert_eq!(pt.x, 2.0);
        assert_eq!(pt.y, 4.0);
        assert_eq!(pt.z, 8.0);
    }

    #[test]
    pub fn test_point3_sub_vector() {
        let left = super::Point3f::new(3.0, 6.0, 9.0);
        let right = Vector3f::new(2.0, 4.0, 8.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    pub fn test_point3_chain() {
        let left = super::Point3f::new(3.0, 6.0, 9.0);
        let right = super::Point3f::new(3.0, 6.0, 9.0);
        let result = ((left + right) / 3.0) * 0.5;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    pub fn test_point3_basic() {
        let left = super::Point2i::new(1, 2, 3);
        let right = super::Point2i::new(3, 4, 5);
        let sum = left + right;
        assert_eq!(sum.x, 4);
        assert_eq!(sum.y, 6);
        assert_eq!(sum.z, 8);
    }

    #[test]
    pub fn test_point3_idx() {
        let left = super::Point3f::new(2.0, 4.0, 6.0);
        assert_eq!(left[0], 2.0);
        assert_eq!(left[1], 4.0);
        assert_eq!(left[2], 6.0);
    }

    #[test]
    pub fn test_point3_div_assign_scalar() {
        let mut left = super::Point3f::new(2.0, 4.0, 6.0);
        left /= 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 3.0);
    }

    #[test]
    pub fn test_point3_mul_assign_scalar() {
        let mut left = super::Point3f::new(2.0, 3.0, 4.0);
        left *= 2.0 as Float;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_point3_div_scalar() {
        let mut left = super::Point3f::new(2.0, 4.0, 6.0);
        left /= 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 3.0);
    }

    #[test]
    pub fn test_point3_mul_scalar() {
        let mut left = super::Point3f::new(2.0, 3.0, 4.0);
        left *= 2.0;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_point3_eq() {
        let left = super::Point3f::new(1.0, 2.0, 3.0);
        let right = super::Point3f::new(1.0, 2.0, 3.0);
        assert!(left == right);
    }

    #[test]
    pub fn test_point3_default() {
        let vec3f = super::Point3f::default();
        assert_eq!(vec3f.x, 0.0);
        assert_eq!(vec3f.y, 0.0);
        assert_eq!(vec3f.z, 0.0);
    }

    #[test]
    pub fn test_point3_new() {
        let vec3f = super::Point3f::new(1.0, 2.0, 3.0);
        assert_eq!(vec3f.x, 1.0);
        assert_eq!(vec3f.y, 2.0);
        assert_eq!(vec3f.z, 3.0);
    }

    #[test]
    pub fn test_point3_nan() {
        let vec3f = super::Point3f {
            x: Float::NAN,
            y: Float::NAN,
            z: Float::NAN,
        };
        assert!(vec3f.has_nan());
    }

    #[test]
    pub fn test_point3_copy() {
        let vec3f = super::Point3f::new(1.0, 2.0, 3.0);
        let cp = vec3f;
        assert_eq!(cp.x, vec3f.x);
        assert_eq!(cp.y, vec3f.y);
        assert_eq!(cp.z, vec3f.z);
    }

    #[test]
    pub fn test_point3_add() {
        let left = super::Point3f::new(1.0, 2.0, 3.0);
        let right = super::Point3f::new(3.0, 4.0, 5.0);
        let result = left + right;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 8.0);
    }

    #[test]
    pub fn test_point3_add_assign() {
        let mut left = super::Point3f::new(1.0, 2.0, 3.0);
        let right = super::Point3f::new(3.0, 4.0, 5.0);
        left += right;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_point3_sub_assign() {
        let mut left = super::Point3f::new(3.0, 6.0, 9.0);
        let right = Vector3f::new(2.0, 4.0, 8.0);
        left -= right;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 1.0);
    }

    #[test]
    pub fn test_point3_sub() {
        let left = super::Point3f::new(3.0, 6.0, 9.0);
        let right = super::Point3f::new(2.0, 4.0, 6.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    pub fn test_point3_neg() {
        let left = super::Point3f::new(3.0, 6.0, 9.0);
        let neg = -left;
        assert_eq!(neg.x, -3.0);
        assert_eq!(neg.y, -6.0);
        assert_eq!(neg.z, -9.0);
    }
}
