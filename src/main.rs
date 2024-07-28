use core::f32;
use std::fs::File;
use std::io::BufWriter;

use crate::camera::Camera;
use crate::hittable::{sphere::Sphere, world::World};
use gfxmath_vec3::{vec3, Vec3};

mod camera;
mod hittable;
mod pixel;
mod ppm;
mod ray;
mod vec3_extension;

fn main() {
    let mut world = World::new();
    world.add_object(Box::new(Sphere::new(vec3!(0.2, 1.0, -3.0), 0.6)));
    world.add_object(Box::new(Sphere::new(vec3!(0.0, 0.0, -3.0), 0.5)));
    world.add_object(Box::new(Sphere::new(vec3!(1.0, 0.0, -3.0), 0.2)));

    const RATIO: f32 = 16.0 / 9.0;
    const VIEWPORT_WIDTH: f32 = 2.0;
    const VIEWPORT_HEIGHT: f32 = VIEWPORT_WIDTH / RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    let camera_center = vec3!(0.0, 0.0, 0.0);
    let camera = Camera::new(
        &camera_center,
        FOCAL_LENGTH,
        (VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    );

    let image = camera.render(&world, 512);

    let mut file_writer = BufWriter::new(File::create("./output/test.ppm").unwrap());
    image.write_to(&mut file_writer).unwrap();
}
