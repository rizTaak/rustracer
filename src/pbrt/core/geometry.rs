use crate::pbrt;
use crate::pbrt::HasNaN;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T: HasNaN> HasNaN for Vector2<T> {
    fn has_nan(&self) -> bool {
        return self.x.has_nan() || self.y.has_nan();
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Vector2<T>) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl<T: PartialEq> Eq for Vector2<T> {}

impl<T: Add<Output = T> + HasNaN> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        debug_assert!(!self.has_nan());
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub type Vector2f = Vector2<pbrt::Float>;
pub type Vector2i = Vector2<i32>;

#[cfg(test)]
mod tests {
    use crate::pbrt;
    use crate::pbrt::HasNaN;

    #[test]
    pub fn test_vector2_eq() {
        let left = super::Vector2f::new(1.0, 2.0);
        let right = super::Vector2f::new(1.0, 2.0);
        assert_eq!(left == right, true);
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
        let vec2f = super::Vector2f::new(pbrt::Float::NAN, pbrt::Float::NAN);
        assert_eq!(vec2f.has_nan(), true);
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
}
