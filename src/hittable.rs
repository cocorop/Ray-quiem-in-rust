use crate::{ray::Ray, vec3::Vec3};

// Stores information about a ray-hittable collision.
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

// Hittable trait to be implemented for all objects
// that a ray can interact with.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
