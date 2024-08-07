use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufReader, Read};

use std::sync::{OnceLock, RwLock};

use gfxmath_vec3::{vec3, Vec3};

use crate::hittable::world::World;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub(super) fn ray_color(ray: &Ray<f32>, world: &World, remaining_depth: u32) -> Vec3<f32> {
    if remaining_depth == 0 {
        return vec3!(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, (0.001, 1000.0)) {
        let reflected_ray = randomly_reflected_ray(&record);
        return 0.5 * ray_color(&reflected_ray, world, remaining_depth - 1);
    }

    let y = ray.unit_direction().y;
    let a = (y - (-1.0)) / (1.0 - (-1.0));
    let (white, blue) = (vec3!(1.0, 1.0, 1.0), vec3!(0.5, 0.7, 1.0));
    let blended = (1.0 - a) * &white + a * &blue;
    blended
}

fn randomly_reflected_ray(hit_record: &HitRecord) -> Ray<f32> {
    let origin = hit_record.ray.at(hit_record.t);
    let direction = random_normal_vector() + &hit_record.surface_normal;
    Ray::new(origin, direction)
}

fn random_normal_vector() -> Vec3<f32> {
    let theta = random() * 2.0 * PI;
    let phi = (random() - 0.5) * PI;

    vec3!(phi.cos() * theta.cos(), phi.cos() * theta.sin(), phi.sin())
}

fn random() -> f32 {
    let r = RANDOM.get_or_init(|| RwLock::new(Random::new()));
    r.write().unwrap().random()
}

static RANDOM: OnceLock<RwLock<Random>> = OnceLock::new();

struct Random {
    reader: BufReader<File>,
}

impl Random {
    fn new() -> Self {
        let f = File::open("/dev/urandom").unwrap();
        Self {
            reader: BufReader::new(f),
        }
    }

    fn random(&mut self) -> f32 {
        let mut buffer = [0; 4];
        self.reader.read_exact(&mut buffer).unwrap();
        let random_int = u32::from_le_bytes(buffer);
        random_int as f32 / u32::MAX as f32
    }
}
