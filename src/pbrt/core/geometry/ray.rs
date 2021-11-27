use crate::pbrt::{Float, Medium, Point3f, Vector3f};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray<'a> {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: Float,
    pub time: Float,
    pub medium: Option<&'a Medium>,
}

impl<'a> Ray<'a> {
    pub fn new(
        o: Point3f,
        d: Vector3f,
        t_max: Option<Float>,
        time: Option<Float>,
        medium: Option<&'a Medium>,
    ) -> Self {
        Ray {
            o,
            d,
            t_max: t_max.unwrap_or(Float::INFINITY),
            time: time.unwrap_or(Float::default()),
            medium,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::pbrt::{Float, Point3f, Vector3f};

    #[test]
    pub fn test_new() {
        let ray = super::Ray::new(Point3f::new(1., 2., 3.),
                                  Vector3f::new(4., 5., 6.),
                                  Option::None,
                                  Option::None, Option::None);
        assert_eq!(ray.o, Point3f::new(1., 2., 3.));
        assert_eq!(ray.d, Vector3f::new(4., 5., 6.));
        assert_eq!(ray.t_max, Float::INFINITY);
        assert_eq!(ray.time, Float::default());
        assert_eq!(ray.medium.is_none(), true);
    }
}
