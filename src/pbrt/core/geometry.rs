use crate::pbrt;

#[derive(Debug, Default)]
pub struct Vector2<T>
where
    T: Default,
{
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Default,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

pub type Vector2f = Vector2<pbrt::Float>;
pub type Vector2i = Vector2<i32>;

impl pbrt::HasNaN for Vector2f {
    fn has_nans(&self) -> bool {
        return self.x.is_nan() || self.y.is_nan();
    }
}

impl pbrt::HasNaN for Vector2i {
    fn has_nans(&self) -> bool {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::pbrt;
    use crate::pbrt::HasNaN;
    
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
        assert_eq!(vec2f.has_nans(), true);
    }
}
