use core::f32;
use std::fs::File;
use std::io::BufWriter;

use gfxmath_vec3::{
    ops::{Dot, Norm},
    vec3, Vec3,
};
use ppm::PPM;
use ray::Ray;

use crate::vec3_extension::Vec3Extension;

mod pixel;
mod ppm;
mod ray;
mod vec3_extension;

fn first_intersection_with_sphere(ray: &Ray<f32>, center: &Vec3<f32>, radius: f32) -> Option<f32> {
    let origin_to_center = center - ray.origin();

    // Solve quadratic equation.
    let a = ray.unit_direction().dot(ray.unit_direction());
    let b = -2.0 * ray.unit_direction().dot(&origin_to_center);
    let c = (&origin_to_center).dot(&origin_to_center) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let smaller_root = (-b - discriminant.sqrt()) / (2.0 * a);
        Some(smaller_root)
    } else {
        None
    }
}

fn ray_color(ray: &Ray<f32>) -> Vec3<f32> {
    let center = vec3!(0.0, 0.0, -1.0);
    let radius = 0.45;

    if let Some(t) = first_intersection_with_sphere(ray, &center, radius) {
        let surface_normal = (ray.at(t) - center).norm().unwrap();
        return (0.5 * surface_normal + vec3!(0.5, 0.5, 0.5)).into();
    }

    let y = ray.unit_direction().y;
    let a = (y - (-1.0)) / (1.0 - (-1.0));
    let (white, blue) = (vec3!(1.0, 1.0, 1.0), vec3!(0.5, 0.7, 1.0));
    let blended = (1.0 - a) * &white + a * &blue;
    blended
}

fn main() {
    const RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 512;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / RATIO) as u32;
    const VIEWPORT_WIDTH: f32 = 2.0;
    const VIEWPORT_HEIGHT: f32 = VIEWPORT_WIDTH * (IMAGE_HEIGHT as f32 / IMAGE_WIDTH as f32);
    const FOCAL_LENGTH: f32 = 1.0;

    let viewport_u = VIEWPORT_WIDTH * Vec3::unit_x();
    let viewport_v = -VIEWPORT_HEIGHT * Vec3::unit_y();
    let delta_pixel_u = &viewport_u / IMAGE_WIDTH as f32;
    let delta_pixel_v = &viewport_v / IMAGE_HEIGHT as f32;

    let camera_center = Vec3::zero();
    let viewport_upper_left = &camera_center
        + -FOCAL_LENGTH * Vec3::unit_z()
        + (-VIEWPORT_WIDTH / 2.0) * Vec3::unit_x()
        + (VIEWPORT_HEIGHT / 2.0) * Vec3::unit_y();
    let viewport_upper_left_pixel_position =
        &viewport_upper_left + 0.5 * &delta_pixel_u + 0.5 * &delta_pixel_v;

    println!("camera: {:?}", camera_center);
    println!("viewport LU: {:?}", viewport_upper_left);
    println!(
        "viewport LU pixel: {:?}",
        viewport_upper_left_pixel_position
    );

    let mut image = PPM::new_black(IMAGE_WIDTH, IMAGE_HEIGHT);
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let pixel_position = &viewport_upper_left_pixel_position
                + x as f32 * &delta_pixel_u
                + y as f32 * &delta_pixel_v;
            let ray_direction = &pixel_position - &camera_center;
            let ray = Ray::new(camera_center.to_owned(), ray_direction);
            image.set_pixel(x, y, ray_color(&ray).into());
        }
    }

    let mut file_writer = BufWriter::new(File::create("./test.ppm").unwrap());
    image.write_to(&mut file_writer).unwrap();
}
