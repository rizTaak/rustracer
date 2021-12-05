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
    use crate::pbrt::{Float, Max, Min, Point2};

    #[test]
    pub fn test_bounds_new() {
        let pt = super::Bounds2::<Float>::new();
        assert_eq!(pt.p_min.x, Float::max_value());
        assert_eq!(pt.p_min.y, Float::max_value());
        assert_eq!(pt.p_max.x, Float::min_value());
        assert_eq!(pt.p_max.y, Float::min_value());
    }

    #[test]
    pub fn test_from_pts() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let bound = super::Bounds2::<Float>::from_pts(pt1, pt2);
        assert_eq!(bound.p_min, Point2::<Float>::new(1.0, 2.0));
        assert_eq!(bound.p_max, Point2::<Float>::new(12.0, 11.0));
    }

    #[test]
    pub fn test_diagnol() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        let diag = b.diagonal();
        assert_eq!(diag.x, 11.0);
        assert_eq!(diag.y, 9.0);
    }

    #[test]
    pub fn test_area() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        let area = b.area();
        assert_eq!(area, 9.0*11.0);
    }

    #[test]
    pub fn test_max_extent_0() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        let extent = b.maximum_extent();
        assert_eq!(extent, 0);
    }

    #[test]
    pub fn test_max_extent_1() {
        let pt1 = Point2::<Float>::new(2.0, 12.0);
        let pt2 = Point2::<Float>::new(11.0, 1.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        let extent = b.maximum_extent();
        assert_eq!(extent, 1);
    }

    #[test]
    pub fn test_idx() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        assert_eq!(b[0], Point2::<Float>::new(1.0, 2.0));
        assert_eq!(b[1], Point2::<Float>::new(12.0, 11.0));
    }

    #[test]
    #[should_panic]
    pub fn test_idx_panic() {
        let pt1 = Point2::<Float>::new(1.0, 11.0);
        let pt2 = Point2::<Float>::new(12.0, 2.0);
        let b = super::Bounds2::<Float>::from_pts(pt1, pt2);
        assert_eq!(b[2], Point2::<Float>::new(1.0, 2.0));
    }

}