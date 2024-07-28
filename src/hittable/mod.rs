use gfxmath_vec3::Vec3;

use crate::ray::Ray;

pub mod sphere;
pub mod world;

pub struct HitRecord {
    pub t: f32,
    pub ray: Ray<f32>,
    pub surface_normal: Vec3<f32>,
}

pub trait HittableObject {
    fn hit(&self, ray: &Ray<f32>) -> Option<HitRecord>;
}
