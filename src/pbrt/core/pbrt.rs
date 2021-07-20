#[cfg(PBRT_FLOAT_AS_DOUBLE)]
pub type Float = f64;

#[cfg(not(PBRT_FLOAT_AS_DOUBLE))]
pub type Float = f32;

pub trait HasNaN {
    fn has_nan(&self) -> bool;
}

impl HasNaN for i32 {
    fn has_nan(&self) -> bool {
        false
    }
}

impl HasNaN for i64 {
    fn has_nan(&self) -> bool {
        false
    }
}

impl HasNaN for f32 {
    fn has_nan(&self) -> bool {
        self.is_nan()
    }
}

impl HasNaN for f64 {
    fn has_nan(&self) -> bool {
        self.is_nan()
    }
}
