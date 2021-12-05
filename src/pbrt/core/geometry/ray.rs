use crate::pbrt::{Float, HasNaN, Medium, Point3f, Vector3f};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray<'a> {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: Float,
    pub time: Float,
    pub medium: Option<&'a Medium>,
}

impl<'a> Ray<'a> {
    pub fn from_od(o: &Point3f, d: &Vector3f) -> Self {
        Self::new(o, d, None, None, None)
    }

    pub fn new(
        o: &Point3f,
        d: &Vector3f,
        t_max: Option<Float>,
        time: Option<Float>,
        medium: Option<&'a Medium>,
    ) -> Self {
        Ray {
            o: *o,
            d: *d,
            t_max: t_max.unwrap_or(Float::INFINITY),
            time: time.unwrap_or(Float::default()),
            medium,
        }
    }

    pub fn has_nan(&self) -> bool {
        self.o.has_nan() || self.d.has_nan() || Float::is_nan(self.t_max)
    }
}

impl<'a> FnOnce<(Float,)> for Ray<'a> {
    type Output = Point3f;

    extern "rust-call" fn call_once(self, args: (Float,)) -> Self::Output {
        self.call(args)
    }
}

impl<'a> FnMut<(Float,)> for Ray<'a> {
    extern "rust-call" fn call_mut(&mut self, args: (Float,)) -> Self::Output {
        self.call(args)
    }
}

impl<'a> Fn<(Float,)> for Ray<'a> {
    extern "rust-call" fn call(&self, args: (Float,)) -> Self::Output {
        let (t,) = args;
        self.o + self.d * t
    }
}

#[cfg(test)]
mod tests {
    use crate::pbrt::{Float, Point3f, Vector3f};

    #[test]
    pub fn test_from_od() {
        let ray = super::Ray::from_od(&Point3f::new(1., 2., 3.), &Vector3f::new(4., 5., 6.));
        assert_eq!(ray.o, Point3f::new(1., 2., 3.));
        assert_eq!(ray.d, Vector3f::new(4., 5., 6.));
        assert_eq!(ray.t_max, Float::INFINITY);
        assert_eq!(ray.time, Float::default());
        assert_eq!(ray.medium.is_none(), true);
    }

    #[test]
    pub fn test_new() {
        let ray = super::Ray::new(
            &Point3f::new(1., 2., 3.),
            &Vector3f::new(4., 5., 6.),
            Option::None,
            Option::None,
            Option::None,
        );
        assert_eq!(ray.o, Point3f::new(1., 2., 3.));
        assert_eq!(ray.d, Vector3f::new(4., 5., 6.));
        assert_eq!(ray.t_max, Float::INFINITY);
        assert_eq!(ray.time, Float::default());
        assert_eq!(ray.medium.is_none(), true);
    }

    #[test]
    pub fn test_no_nan() {
        let ray = super::Ray::new(
            &Point3f::new(1., 2., 3.),
            &Vector3f::new(4., 5., 6.),
            Option::None,
            Option::None,
            Option::None,
        );
        assert_eq!(ray.has_nan(), false)
    }

    #[test]
    pub fn test_has_nan() {
        let ray = super::Ray::new(
            &Point3f::new(1., 2., 3.),
            &Vector3f::new(4., 5., 6.),
            Option::Some(Float::NAN),
            Option::None,
            Option::None,
        );
        assert_eq!(ray.has_nan(), true)
    }

    #[test]
    pub fn test_interpol() {
        let ray = super::Ray::new(
            &Point3f::new(1., 2., 3.),
            &Vector3f::new(2., 2., 2.),
            Option::None,
            Option::None,
            Option::None,
        );
        let interpol = ray(0.5);
        assert_eq!(interpol, Point3f::new(2., 3., 4.));
        let interpol = ray(2.0);
        assert_eq!(interpol, Point3f::new(5., 6., 7.));
    }

    #[test]
    pub fn test_interpol_ref() {
        let ray = &super::Ray::new(
            &Point3f::new(1., 2., 3.),
            &Vector3f::new(2., 2., 2.),
            Option::None,
            Option::None,
            Option::None,
        );
        let interpol = ray(0.5);
        assert_eq!(interpol, Point3f::new(2., 3., 4.));
        let interpol = ray(2.0);
        assert_eq!(interpol, Point3f::new(5., 6., 7.));
    }
}
