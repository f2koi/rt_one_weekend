use crate::{hittable::HittableObject, ray::Ray};

use crate::hittable::HitRecord;

pub struct World {
    objects: Vec<Box<dyn HittableObject>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn HittableObject>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray<f32>, interval: (f32, f32)) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray))
            .filter(|hit_record| interval.0 <= hit_record.t && hit_record.t <= interval.1)
            .min_by(|r1, r2| r1.t.partial_cmp(&r2.t).unwrap())
    }
}
