use crate::pbrt::{Float, One};

#[inline]
pub fn lerp(t: Float, s: Float, e: Float) -> Float {
    (Float::one() - t) * s + t * e
}
