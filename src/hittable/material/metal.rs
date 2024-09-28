use gfxmath_vec3::ops::Dot;
use gfxmath_vec3::Vec3;

use crate::hittable::HitRecord;
use crate::hittable::material::{Material, Reflection};
use crate::ray::Ray;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3<f32>,
}

impl Metal {
    pub fn new(albedo: Vec3<f32>) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &HitRecord) -> Option<Reflection> {
        let n = &hit_record.surface_normal;
        let v = hit_record.ray.unit_direction();

        let reflected_direction = v - 2.0 * v.dot(n) * n;
        let origin = hit_record.ray.at(hit_record.t);

        let reflected_ray = Ray::new(origin, reflected_direction);
        Some(Reflection {
            attenuation: self.albedo.to_owned(),
            ray: reflected_ray,
        })
    }
}
