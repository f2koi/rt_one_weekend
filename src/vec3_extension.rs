use gfxmath_vec3::{vec3, Vec3};

pub trait Vec3Extension<T> {
    fn unit_x() -> Vec3<T>;
    fn unit_y() -> Vec3<T>;
    fn unit_z() -> Vec3<T>;
    fn zero() -> Vec3<T>;
}

impl Vec3Extension<f32> for Vec3<f32> {
    fn unit_x() -> Vec3<f32> {
        vec3!(1.0, 0.0, 0.0)
    }

    fn unit_y() -> Vec3<f32> {
        vec3!(0.0, 1.0, 0.0)
    }

    fn unit_z() -> Vec3<f32> {
        vec3!(0.0, 0.0, 1.0)
    }

    fn zero() -> Vec3<f32> {
        vec3!(0.0, 0.0, 0.0)
    }
}
