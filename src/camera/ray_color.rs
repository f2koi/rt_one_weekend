use gfxmath_vec3::{vec3, Vec3};

use crate::hittable::material::{ConcreteMaterial, Material};
use crate::hittable::world::World;
use crate::ray::Ray;

pub(super) fn ray_color(ray: &Ray<f32>, world: &World, remaining_depth: u32) -> Vec3<f32> {
    if remaining_depth == 0 {
        return vec3!(0.0, 0.0, 0.0);
    }

    if let Some(ref record) = world.hit(ray, (0.001, 1000.0)) {
        let reflection = match &record.material {
            ConcreteMaterial::Lambertian(lambertian) => lambertian.scatter(record),
            ConcreteMaterial::Metal(metal) => metal.scatter(record),
        };

        // reflection
        if let Some(reflection) = reflection {
            return reflection.attenuation * ray_color(&reflection.ray, world, remaining_depth - 1);
        }
        
        // absortion
        return vec3!(0.0, 0.0, 0.0);
    }

    // sky
    let y = ray.unit_direction().y;
    let a = (y - (-1.0)) / (1.0 - (-1.0));
    let (white, blue) = (vec3!(1.0, 1.0, 1.0), vec3!(0.5, 0.7, 1.0));
    let blended = (1.0 - a) * &white + a * &blue;
    blended
}
