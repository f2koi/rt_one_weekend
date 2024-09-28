use gfxmath_vec3::Vec3;
use crate::ray::Ray;

use super::HitRecord;

pub mod lambertian;
pub mod metal;

pub struct Reflection {
    pub attenuation: Vec3<f32>,
    pub ray: Ray<f32>,
}

pub trait Material {
    fn scatter(&self, hit_record: &HitRecord) -> Option<Reflection>;
}

#[derive(Clone)]
pub enum ConcreteMaterial {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
}
