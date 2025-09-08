use crate::{ray::Ray, vec3::Vec3};

// Stores information about a ray-hittable collision.
#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub facing_front: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord::default()
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.facing_front = ray.direction * outward_normal < 0.0;
        self.normal = if self.facing_front {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

// Hittable trait to be implemented for all objects
// that a ray can interact with.
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
