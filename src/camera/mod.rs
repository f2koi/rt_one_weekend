use gfxmath_vec3::{vec3, Vec3};
use std::borrow::Borrow;

use gfxmath_vec3::ops::{Cross, Norm};

use crate::camera::point_sampler::{UniformPointSampler, PointSampler};
use crate::camera::ray_color::ray_color;
use crate::camera::viewport::Viewport;
use crate::hittable::world::World;
use crate::ppm::PPM;
use crate::ray::Ray;
use crate::vec3_extension::Vec3Extension;

mod point_sampler;
mod ray_color;
pub mod viewport;

pub struct Camera {
    center: Vec3<f32>,
    focal_vector: Vec3<f32>,
    top_direction: Vec3<f32>,
    viewport: Viewport,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(
        center: &Vec3<f32>,
        focal_vector: Vec3<f32>,
        top_direction: Vec3<f32>,
        viewport: Viewport,
    ) -> Self {
        Self {
            center: center.to_owned(),
            focal_vector,
            top_direction,
            viewport,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.viewport.width / self.viewport.height
    }

    pub fn render(&self, world: &World, image_width: u32) -> PPM {
        let image_height = (image_width as f32 / self.ratio()) as u32;

        let unit_z = -self.focal_vector.borrow().norm().unwrap();
        let unit_y = self.top_direction.to_owned();
        let unit_x = unit_y.borrow().cross(&unit_z);

        let viewport_u = self.viewport.width * &unit_x;
        let viewport_v = -self.viewport.height * &unit_y;
        let delta_pixel_u = &viewport_u / image_width as f32;
        let delta_pixel_v = &viewport_v / image_height as f32;

        let viewport_upper_left = &self.center
            + -self.focal_vector.l2norm() * unit_z
            + (-self.viewport.width / 2.0) * unit_x
            + (self.viewport.height / 2.0) * unit_y;
        let viewport_upper_left_pixel_position =
            &viewport_upper_left + 0.5 * &delta_pixel_u + 0.5 * &delta_pixel_v;

        let point_sampler = UniformPointSampler::new(&delta_pixel_u, &delta_pixel_v);

        let mut image = PPM::new_black(image_width, image_height);
        for x in 0..image_width {
            for y in 0..image_height {
                let pixel_position = &viewport_upper_left_pixel_position
                    + x as f32 * &delta_pixel_u
                    + y as f32 * &delta_pixel_v;

                let mut blended_color: Vec3<f32> = vec3!(0.0, 0.0, 0.0);

                let sample_points = point_sampler.sample(&pixel_position);
                for sample_point in sample_points.iter() {
                    let ray_direction = sample_point - &self.center;
                    let ray = Ray::new(self.center.to_owned(), ray_direction);
                    let color = ray_color(&ray, world, 10);
                    blended_color += color;
                }

                blended_color /= sample_points.len() as f32;

                image.set_pixel(x, y, blended_color.into());
            }
        }

        image
    }
}
