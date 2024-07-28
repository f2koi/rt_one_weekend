use gfxmath_vec3::{ops::Norm, vec3, Vec3};

#[derive(Clone, Debug)]
pub struct Ray<T> {
    origin: Vec3<T>,
    unit_direction: Vec3<T>,
}

impl Ray<f32> {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Self {
        assert_ne!(direction, vec3!(0.0, 0.0, 0.0));
        Self {
            origin,
            unit_direction: direction.norm().unwrap(),
        }
    }

    pub fn at(&self, time: f32) -> Vec3<f32> {
        &self.origin + time * &self.unit_direction
    }

    pub fn unit_direction(&self) -> &Vec3<f32> {
        &self.unit_direction
    }

    pub fn origin(&self) -> &Vec3<f32> {
        &self.origin
    }
}
