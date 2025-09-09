pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}