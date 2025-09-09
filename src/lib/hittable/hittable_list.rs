use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::<Box<dyn Hittable>>::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_something = false;

        let mut temp_record = HitRecord::new();
        let mut closest_time = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_time, &mut temp_record) {
                hit_something = true;
                closest_time = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }

        hit_something
    }
}
