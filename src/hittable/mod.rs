use gfxmath_vec3::Vec3;

use material::ConcreteMaterial;
use crate::ray::Ray;

pub mod sphere;
pub mod world;
pub mod material;

pub struct HitRecord {
    pub t: f32,
    pub ray: Ray<f32>,
    pub surface_normal: Vec3<f32>,
    pub material: ConcreteMaterial,
}

pub trait HittableObject {
    fn hit(&self, ray: &Ray<f32>) -> Option<HitRecord>;
}
