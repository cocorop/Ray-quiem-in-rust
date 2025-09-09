use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        assert!(radius > 0.0, "Cannot create a sphere with a negative or zero radius.");
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let cmq = self.center - ray.origin;

        let a = ray.direction * ray.direction;
        let h = ray.direction * cmq;
        let c = cmq * cmq - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let d_sqrt = discriminant.sqrt();

        let mut root = (h - d_sqrt) / a;
        if t_min >= root || root >= t_max {
            root = (h + d_sqrt) / a;
            if t_min >= root || root >= t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        true
    }
}
