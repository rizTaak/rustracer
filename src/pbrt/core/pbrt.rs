#[cfg(PBRT_FLOAT_AS_DOUBLE)]
pub type Float = f64;

#[cfg(not(PBRT_FLOAT_AS_DOUBLE))]
pub type Float = f32;

pub trait HasNaN {
    fn has_nans(&self) -> bool;
}
