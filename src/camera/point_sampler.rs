use gfxmath_vec3::Vec3;

pub(super) struct PointSampler {
    delta_pixel_u: Vec3<f32>,
    delta_pixel_v: Vec3<f32>,
}

impl PointSampler {
    pub(super) fn new(delta_pixel_u: &Vec3<f32>, delta_pixel_v: &Vec3<f32>) -> Self {
        Self {
            delta_pixel_u: delta_pixel_u.to_owned(),
            delta_pixel_v: delta_pixel_v.to_owned(),
        }
    }

    pub(super) fn sample(&self, pixel_point: &Vec3<f32>) -> Vec<Vec3<f32>> {
        let mut sample_points = vec![];
        for i in 0..3 {
            for j in 0..3 {
                let sampled = pixel_point
                    + (i as f32 / 3.0) * &self.delta_pixel_u
                    + (j as f32 / 3.0) * &self.delta_pixel_v;
                sample_points.push(sampled);
            }
        }
        sample_points
    }
}
