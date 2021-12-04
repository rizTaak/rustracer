use std::ops::Index;

use crate::pbrt::{Int, Point2, Scalar, Vector2};

#[derive(Debug, Default, Copy, Clone)]
pub struct Bounds2<T: Scalar> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl<T: Scalar> Bounds2<T> {
    pub fn new() -> Self {
        Self {
            p_min: Point2::<T>::new(T::max_value(), T::max_value()),
            p_max: Point2::<T>::new(T::min_value(), T::min_value()),
        }
    }

    pub fn from_pts(p1: Point2<T>, p2: Point2<T>) -> Self {
        Self {
            p_min: Point2::<T>::new(T::min(p1.x, p2.x), T::min(p1.y, p2.y)),
            p_max: Point2::<T>::new(T::max(p1.x, p2.x), T::max(p1.y, p2.y)),
        }
    }

    pub fn diagonal(&self) -> Vector2<T> {
        self.p_max - self.p_min
    }

    pub fn area(&self) -> T {
        let d = self.p_max - self.p_min;
        d.x * d.y
    }

    pub fn maximum_extent(&self) -> Int {
        let diagonal = self.diagonal();
        if diagonal.x > diagonal.y { 0 } else { 1 }
    }
}

impl<T: Scalar> Index<Int> for Bounds2<T> {
    type Output = Point2<T>;

    fn index(&self, idx: Int) -> &Self::Output {
        debug_assert!(0 <= idx && idx <= 1);
        match idx {
            0 => &self.p_min,
            1 => &self.p_max,
            _ => panic!("index {} is used to access Bounds2<T> scalar", idx),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::pbrt::{Float, Max, Min};

    #[test]
    pub fn test_bounds_new() {
        let pt = super::Bounds2::<Float>::new();
        assert_eq!(pt.p_min.x, Float::max_value());
        assert_eq!(pt.p_min.y, Float::max_value());
        assert_eq!(pt.p_max.x, Float::min_value());
        assert_eq!(pt.p_max.y, Float::min_value());
    }
}