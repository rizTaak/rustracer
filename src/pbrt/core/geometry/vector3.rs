use crate::pbrt;
use crate::pbrt::Scalar;
use crate::pbrt::HasNaN;
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
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        debug_assert!(!x.has_nan());
        debug_assert!(!y.has_nan());
        debug_assert!(!z.has_nan());
        Self { x: x, y: y, z: z }
    }

    #[allow(dead_code)]
    pub fn length_squared(&self) -> pbrt::Float {
        let squared = self.x * self.x + self.y * self.y + self.z * self.z;
        squared.to_float()
    }

    #[allow(dead_code)]
    pub fn length(&self) -> pbrt::Float {
        self.length_squared().sqrt()
    }
}

impl<T: Scalar> HasNaN for Vector3<T> {
    fn has_nan(&self) -> bool {
        return self.x.has_nan() || self.y.has_nan() || self.z.has_nan();
    }
}

impl<T: Scalar> PartialEq for Vector3<T> {
    fn eq(&self, rhs: &Vector3<T>) -> bool {
        return self.x == rhs.x && self.y == rhs.y && self.z == self.z;
    }
}

impl<T: Scalar> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(!self.has_nan());
        return Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl<T: Scalar> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl<T: Scalar> Sub for Vector3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(!rhs.has_nan());
        return Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl<T: Scalar> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl<T: Scalar> Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        debug_assert!(!rhs.has_nan());
        return Self::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl<T: Scalar> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl<T: Scalar> Div<T> for Vector3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        debug_assert_ne!(rhs, T::zero());
        let inv = T::one() / rhs;
        return Self::new(self.x * inv, self.y * inv, self.z * inv);
    }
}

impl<T: Scalar> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        debug_assert!(!rhs.has_nan());
        *self = Self::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl<T: Scalar> Neg for Vector3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        return Self::new(-self.x, -self.y, -self.z);
    }
}

impl<T: Scalar> Index<pbrt::Int> for Vector3<T> {
    type Output = T;

    fn index(&self, idx: pbrt::Int) -> &Self::Output {
        debug_assert!(0 <= idx && idx <= 2);
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index {} to access Vector3 Scalar", idx),
        }
    }
}

#[allow(dead_code)]
pub type Vector3f = Vector3<pbrt::Float>;
#[allow(dead_code)]
pub type Vector3i = Vector3<i32>;

#[cfg(test)]
mod tests {
    use crate::pbrt;
    use crate::pbrt::HasNaN;
    use crate::pbrt::Float;

    #[test]
    pub fn tst_vector3_chain() {
        let left = super::Vector3f::new(3.0, 6.0, 9.0);
        let right = super::Vector3f::new(3.0, 6.0, 9.0);
        let result = ((left + right) / 3.0) * 0.5;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    pub fn test_vector3_length() {
        let left = super::Vector3f::new(3.0, 4.0, 5.0);
        let length = left.length();
        assert_eq!(pbrt::Float::trunc(length * 1000000.0) / 1000000.0, 7.071067);
    }

    #[test]
    pub fn test_vector3i_basic() {
        let left = super::Vector3i::new(1, 2, 3);
        let right = super::Vector3i::new(3, 4, 5);
        let sum = left + right;
        assert_eq!(sum.x, 4);
        assert_eq!(sum.y, 6);
        assert_eq!(sum.z, 8);
    }

    #[test]
    pub fn test_vector3_idx() {
        let left = super::Vector3f::new(2.0, 4.0, 6.0);
        assert_eq!(left[0], 2.0);
        assert_eq!(left[1], 4.0);
        assert_eq!(left[2], 6.0);
    }

    #[test]
    pub fn test_vector3_div_assign_scalar() {
        let mut left = super::Vector3f::new(2.0, 4.0, 8.0);
        left /= 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 4.0);
    }

    #[test]
    pub fn test_vector3_mul_assign_scalar() {
        let mut left = super::Vector3f::new(2.0, 3.0, 4.0);
        left *= 2.0 as Float;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_vector3_div_scalar() {
        let mut left = super::Vector3f::new(2.0, 4.0, 8.0);
        left = left / 2.0 as Float;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 4.0);
    }

    #[test]
    pub fn test_vector3_mul_scalar() {
        let mut left = super::Vector3f::new(2.0, 3.0, 4.0);
        left = left * 2.0;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_vector3_eq() {
        let left = super::Vector3f::new(1.0, 2.0, 4.0);
        let right = super::Vector3f::new(1.0, 2.0, 4.0);
        assert_eq!(left == right, true);
    }

    #[test]
    pub fn test_vector3f_default() {
        let vec3f = super::Vector3f::default();
        assert_eq!(vec3f.x, 0.0);
        assert_eq!(vec3f.y, 0.0);
        assert_eq!(vec3f.z, 0.0);
    }

    #[test]
    pub fn test_vector3f_new() {
        let vec3f = super::Vector3f::new(1.0, 2.0, 3.0);
        assert_eq!(vec3f.x, 1.0);
        assert_eq!(vec3f.y, 2.0);
        assert_eq!(vec3f.z, 3.0);
    }

    #[test]
    pub fn test_vector3f_nan() {
        let vec3f = super::Vector3f {
            x: pbrt::Float::NAN,
            y: pbrt::Float::NAN,
            z: pbrt::Float::NAN,
        };
        assert_eq!(vec3f.has_nan(), true);
    }

    #[test]
    pub fn test_vector3f_copy() {
        let vec3f = super::Vector3f::new(1.0, 2.0, 3.0);
        let cp = vec3f;
        assert_eq!(cp.x, vec3f.x);
        assert_eq!(cp.y, vec3f.y);
        assert_eq!(cp.z, vec3f.z);
    }

    #[test]
    pub fn test_vector3f_add() {
        let left = super::Vector3f::new(1.0, 2.0, 3.0);
        let right = super::Vector3f::new(3.0, 4.0, 5.0);
        let result = left + right;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 8.0);
    }

    #[test]
    pub fn test_vector3f_add_assign() {
        let mut left = super::Vector3f::new(1.0, 2.0, 3.0);
        let right = super::Vector3f::new(3.0, 4.0, 5.0);
        left += right;
        assert_eq!(left.x, 4.0);
        assert_eq!(left.y, 6.0);
        assert_eq!(left.z, 8.0);
    }

    #[test]
    pub fn test_vector3f_sub_assign() {
        let mut left = super::Vector3f::new(3.0, 6.0, 9.0);
        let right = super::Vector3f::new(2.0, 4.0, 8.0);
        left -= right;
        assert_eq!(left.x, 1.0);
        assert_eq!(left.y, 2.0);
        assert_eq!(left.z, 1.0);
    }

    #[test]
    pub fn test_vector3f_sub() {
        let left = super::Vector3f::new(3.0, 6.0, 9.0);
        let right = super::Vector3f::new(2.0, 4.0, 8.0);
        let result = left - right;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    pub fn test_vector3f_neg() {
        let left = super::Vector3f::new(3.0, 6.0, 9.0);
        let neg = -left;
        assert_eq!(neg.x, -3.0);
        assert_eq!(neg.y, -6.0);
        assert_eq!(neg.z, -9.0);
    }
}
