use gfxmath_vec3::{vec3, Vec3};

use crate::hittable::world::World;
use crate::ppm::PPM;
use crate::ray::Ray;
use crate::vec3_extension::Vec3Extension;

pub struct Camera {
    center: Vec3<f32>,
    focal_length: f32,
    viewport: (f32, f32),
}

impl Camera {
    fn viewport_width(&self) -> f32 {
        self.viewport.0
    }

    fn viewport_height(&self) -> f32 {
        self.viewport.1
    }
}

#[allow(dead_code)]
impl Camera {
    pub fn new(center: &Vec3<f32>, focal_length: f32, viewport: (f32, f32)) -> Self {
        Self {
            center: center.to_owned(),
            focal_length,
            viewport,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.viewport_width() / self.viewport_height()
    }

    pub fn render(&self, world: &World, image_width: u32) -> PPM {
        let image_height = (image_width as f32 / self.ratio()) as u32;

        let viewport_u = self.viewport_width() * Vec3::unit_x();
        let viewport_v = -self.viewport_height() * Vec3::unit_y();
        let delta_pixel_u = &viewport_u / image_width as f32;
        let delta_pixel_v = &viewport_v / image_height as f32;

        let viewport_upper_left = &self.center
            + -self.focal_length * Vec3::unit_z()
            + (-self.viewport_width() / 2.0) * Vec3::unit_x()
            + (self.viewport_height() / 2.0) * Vec3::unit_y();
        let viewport_upper_left_pixel_position =
            &viewport_upper_left + 0.5 * &delta_pixel_u + 0.5 * &delta_pixel_v;

        let mut image = PPM::new_black(image_width, image_height);
        for x in 0..image_width {
            for y in 0..image_height {
                let pixel_position = &viewport_upper_left_pixel_position
                    + x as f32 * &delta_pixel_u
                    + y as f32 * &delta_pixel_v;
                let ray_direction = pixel_position - &self.center;
                let ray = Ray::new(self.center.to_owned(), ray_direction);
                let color = ray_color(&ray, world);
                image.set_pixel(x, y, color.into());
            }
        }

        image
    }
}

fn ray_color(ray: &Ray<f32>, world: &World) -> Vec3<f32> {
    if let Some(record) = world.hit(ray, (0.0, 1000.0)) {
        return (0.5 * record.surface_normal + vec3!(0.5, 0.5, 0.5)).into();
    }

    let y = ray.unit_direction().y;
    let a = (y - (-1.0)) / (1.0 - (-1.0));
    let (white, blue) = (vec3!(1.0, 1.0, 1.0), vec3!(0.5, 0.7, 1.0));
    let blended = (1.0 - a) * &white + a * &blue;
    blended
}
