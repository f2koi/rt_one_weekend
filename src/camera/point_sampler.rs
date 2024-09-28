use gfxmath_vec3::Vec3;
use rand::{thread_rng, Rng};


pub(super) trait PointSampler {
    fn sample(&self, pixel_point: &Vec3<f32>) -> Vec<Vec3<f32>>;
}

pub(super) struct UniformPointSampler {
    delta_pixel_u: Vec3<f32>,
    delta_pixel_v: Vec3<f32>,
}

impl UniformPointSampler {
    pub fn new(delta_pixel_u: &Vec3<f32>, delta_pixel_v: &Vec3<f32>) -> Self {
        Self {
            delta_pixel_u: delta_pixel_u.to_owned(),
            delta_pixel_v: delta_pixel_v.to_owned(),
        }
    }
}

impl PointSampler for UniformPointSampler {
    fn sample(&self, pixel_point: &Vec3<f32>) -> Vec<Vec3<f32>> {
        let mut sample_points = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                let sampled = pixel_point
                    + (i as f32 / 3.0) * &self.delta_pixel_u
                    + (j as f32 / 3.0) * &self.delta_pixel_v;
                sample_points.push(sampled);
            }
        }
        sample_points
    }
}

pub(super) struct RandomPointSampler {
    delta_pixel_u: Vec3<f32>,
    delta_pixel_v: Vec3<f32>,
}

impl RandomPointSampler {
    pub fn new(delta_pixel_u: &Vec3<f32>, delta_pixel_v: &Vec3<f32>) -> Self {
        Self {
            delta_pixel_u: delta_pixel_u.to_owned(),
            delta_pixel_v: delta_pixel_v.to_owned(),
        }
    }
}

impl PointSampler for RandomPointSampler {
    fn sample(&self, pixel_point: &Vec3<f32>) -> Vec<Vec3<f32>> {
        let mut rng = thread_rng();
        let mut sample_points = Vec::new();
        for _ in 0..9 {
            let i = rng.gen_range(-0.5..0.5);
            let j = rng.gen_range(-0.5..0.5);
            let sampled = pixel_point + i * &self.delta_pixel_u + j * &self.delta_pixel_v;
            sample_points.push(sampled);
        }
        sample_points
    }
}
