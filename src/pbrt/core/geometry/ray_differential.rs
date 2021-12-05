use crate::pbrt::{Float, HasNaN, Point3f, Ray, Vector3f};

#[derive(Debug, Default, Copy, Clone)]
pub struct RayDifferential<'a> {
    pub ray: Ray<'a>,
    pub has_differential: bool,
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vector3f,
    pub ry_direction: Vector3f,
}

impl<'a> RayDifferential<'a> {
    pub fn from_ray(ray: &Ray<'a>) -> Self {
        Self {
            ray: *ray,
            has_differential: false,
            rx_origin: Default::default(),
            ry_origin: Default::default(),
            rx_direction: Default::default(),
            ry_direction: Default::default(),
        }
    }
    pub fn from_od(o: &Point3f, d: &Vector3f) -> Self {
        RayDifferential::new(o, d, false, &None, &None, &None, &None)
    }
    pub fn new(
        o: &Point3f,
        d: &Vector3f,
        has_differential: bool,
        rx_origin: &Option<Point3f>,
        ry_origin: &Option<Point3f>,
        rx_direction: &Option<Vector3f>,
        ry_direction: &Option<Vector3f>,
    ) -> Self {
        Self {
            ray: Ray::new(o, d, None, None, None),
            has_differential,
            rx_origin: rx_origin.unwrap_or(Default::default()),
            ry_origin: ry_origin.unwrap_or(Default::default()),
            rx_direction: rx_direction.unwrap_or(Default::default()),
            ry_direction: ry_direction.unwrap_or(Default::default()),
        }
    }

    pub fn has_nan(&self) -> bool {
        self.ray.has_nan()
            || (self.has_differential
                && (self.rx_origin.has_nan()
                    || self.rx_direction.has_nan()
                    || self.ry_origin.has_nan()
                    || self.ry_direction.has_nan()))
    }

    pub fn scale_differential(&mut self, s: Float) {
        self.rx_origin = self.ray.o + (self.rx_origin - self.ray.o) * s;
        self.ry_origin = self.ray.o + (self.ry_origin - self.ray.o) * s;
        self.rx_direction = self.ray.d + (self.rx_direction - self.ray.d) * s;
        self.ry_direction = self.ray.d + (self.ry_direction - self.ray.d) * s;
    }
}

#[cfg(test)]
mod tests {
    use crate::pbrt::{Point3f, Ray, Vector3f};

    #[test]
    pub fn test_from_ray() {
        let o = &Point3f::new(1., 2., 3.);
        let d = &Vector3f::new(4., 5., 6.);
        let r = &Ray::from_od(o, d);
        let ray_diff = super::RayDifferential::from_ray(r);
        assert_eq!(ray_diff.ray.o, *o);
        assert_eq!(ray_diff.ray.d, *d);
        assert_eq!(ray_diff.has_differential, false);
    }

    #[test]
    pub fn test_has_no_nan() {
        let o = &Point3f::new(1., 2., 3.);
        let d = &Vector3f::new(4., 5., 6.);
        let r = &Ray::from_od(o, d);
        let ray_diff = super::RayDifferential::from_ray(r);
        assert_eq!(false, ray_diff.has_nan())
    }

    #[test]
    pub fn test_from_od() {
        let o = &Point3f::new(1., 2., 3.);
        let d = &Vector3f::new(4., 5., 6.);
        let ray_diff = super::RayDifferential::from_od(o, d);
        assert_eq!(ray_diff.ray.o, *o);
        assert_eq!(ray_diff.ray.d, *d);
        assert_eq!(ray_diff.has_differential, false);
    }

    #[test]
    pub fn test_new() {
        let o = &Point3f::new(1., 2., 3.);
        let d = &Vector3f::new(4., 5., 6.);
        let rx_o = Point3f::new(1., 2., 4.);
        let ry_o = Point3f::new(1., 2., 5.);
        let rx_d = Vector3f::new(4., 5., 7.);
        let ry_d = Vector3f::new(4., 5., 8.);
        let ray_diff = super::RayDifferential::new(
            o,
            d,
            true,
            &Some(rx_o),
            &Some(ry_o),
            &Some(rx_d),
            &Some(ry_d),
        );
        assert_eq!(ray_diff.ray.o, *o);
        assert_eq!(ray_diff.ray.d, *d);
        assert_eq!(ray_diff.has_differential, true);
        assert_eq!(ray_diff.rx_origin, rx_o);
        assert_eq!(ray_diff.ry_origin, ry_o);
        assert_eq!(ray_diff.rx_direction, rx_d);
        assert_eq!(ray_diff.ry_direction, ry_d);
    }

    #[test]
    pub fn test_scale_differential() {
        let o = &Point3f::new(1., 2., 3.);
        let d = &Vector3f::new(4., 5., 6.);
        let rx_o = Point3f::new(1., 2., 4.);
        let ry_o = Point3f::new(1., 2., 5.);
        let rx_d = Vector3f::new(4., 5., 7.);
        let ry_d = Vector3f::new(4., 5., 8.);
        let mut ray_diff = super::RayDifferential::new(
            o,
            d,
            true,
            &Some(rx_o),
            &Some(ry_o),
            &Some(rx_d),
            &Some(ry_d),
        );
        let scale_factor = 2.0;
        let updated_rx_origin =
            ray_diff.ray.o + (ray_diff.rx_origin - ray_diff.ray.o) * scale_factor;
        let updated_ry_origin =
            ray_diff.ray.o + (ray_diff.ry_origin - ray_diff.ray.o) * scale_factor;
        let updated_rx_direction =
            ray_diff.ray.d + (ray_diff.rx_direction - ray_diff.ray.d) * scale_factor;
        let updated_ry_direction =
            ray_diff.ray.d + (ray_diff.ry_direction - ray_diff.ray.d) * scale_factor;

        ray_diff.scale_differential(scale_factor);

        assert_eq!(ray_diff.rx_origin, updated_rx_origin);
        assert_eq!(ray_diff.ry_origin, updated_ry_origin);
        assert_eq!(ray_diff.rx_direction, updated_rx_direction);
        assert_eq!(ray_diff.ry_direction, updated_ry_direction);
    }
}
