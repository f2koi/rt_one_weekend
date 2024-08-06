use gfxmath_vec3::{vec3, Vec3};

use crate::hittable::world::World;
use crate::ray::Ray;

pub(super) fn ray_color(ray: &Ray<f32>, world: &World) -> Vec3<f32> {
    if let Some(record) = world.hit(ray, (0.0, 1000.0)) {
        return (0.5 * record.surface_normal + vec3!(0.5, 0.5, 0.5)).into();
    }

    let y = ray.unit_direction().y;
    let a = (y - (-1.0)) / (1.0 - (-1.0));
    let (white, blue) = (vec3!(1.0, 1.0, 1.0), vec3!(0.5, 0.7, 1.0));
    let blended = (1.0 - a) * &white + a * &blue;
    blended
}
