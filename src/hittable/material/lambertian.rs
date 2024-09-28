use std::f32::consts::PI;

use gfxmath_vec3::{vec3, Vec3};
use rand::random;

use crate::hittable::HitRecord;
use crate::hittable::material::{Material, Reflection};
use crate::ray::Ray;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vec3<f32>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_record: &HitRecord) -> Option<Reflection> {
        let origin = hit_record.ray.at(hit_record.t);
        let direction = random_normal_vector() + 1.01 * &hit_record.surface_normal;
        let reflected_ray = Ray::new(origin, direction);

        Some(Reflection {
            attenuation: self.albedo.to_owned(),
            ray: reflected_ray,
        })
    }
}

fn random_normal_vector() -> Vec3<f32> {
    let theta = random::<f32>() * 2.0 * PI;
    let phi = (random::<f32>() - 0.5) * PI;

    vec3!(phi.cos() * theta.cos(), phi.cos() * theta.sin(), phi.sin())
}
