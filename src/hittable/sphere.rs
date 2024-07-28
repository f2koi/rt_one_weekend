use crate::hittable::{HitRecord, HittableObject};
use crate::ray::Ray;
use crate::vec3_extension::Vec3Extension;

use gfxmath_vec3::ops::{Dot, Norm};
use gfxmath_vec3::Vec3;

pub struct Sphere {
    center: Vec3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl HittableObject for Sphere {
    fn hit(&self, ray: &Ray<f32>) -> Option<HitRecord> {
        let origin_to_center = &self.center - ray.origin();

        // Solve quadratic equation.
        let a = ray.unit_direction().squared();
        let b = -2.0 * ray.unit_direction().dot(&origin_to_center);
        let c = origin_to_center.squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let smaller_root = (-b - discriminant.sqrt()) / (2.0 * a);

        Some(HitRecord {
            t: smaller_root,
            ray: ray.to_owned(),
            surface_normal: (ray.at(smaller_root) - &self.center).norm().unwrap(),
        })
    }
}
