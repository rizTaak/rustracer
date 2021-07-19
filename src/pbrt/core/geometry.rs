use super::pbrt;

#[derive(Debug, Default)]
pub struct Vector2<T> where T: Default {
    pub x: T, 
    pub y: T
}

impl<T> Vector2<T> where T: Default {
    pub fn new(x: T, y: T) -> Self{
        Self { x:x, y:y}
    }
}

pub type Vector2f = Vector2<pbrt::Float>;
pub type Vector2i = Vector2<i32>;


mod tests {
    #[test]
    pub fn test_vector2_default() {
        let vec2f = super::Vector2f::default();
        assert_eq!(vec2f.x, 0.0);
        assert_eq!(vec2f.y, 0.0);
    }

    #[test]
    pub fn test_vector2_new() {
        let vec2f = super::Vector2f::new(1.0, 2.0);
        assert_eq!(vec2f.x, 1.0);
        assert_eq!(vec2f.y, 2.0);
    }
}

