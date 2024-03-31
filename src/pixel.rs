use gfxmath_vec3::{vec3, Vec3};

#[derive(Clone)]
pub struct Pixel {
    vec3: Vec3<u8>,
}

impl From<Vec3<f32>> for Pixel {
    fn from(value: Vec3<f32>) -> Self {
        Self {
            vec3: vec3!(
                (value.x * 256.0) as u8,
                (value.y * 256.0) as u8,
                (value.z * 256.0) as u8
            ),
        }
    }
}

impl From<Vec3<u8>> for Pixel {
    fn from(value: Vec3<u8>) -> Self {
        Self { vec3: value }
    }
}

#[allow(dead_code)]
impl Pixel {
    pub fn red(&self) -> u8 {
        self.vec3.x
    }
    pub fn green(&self) -> u8 {
        self.vec3.y
    }

    pub fn blue(&self) -> u8 {
        self.vec3.z
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            vec3: vec3!(red, green, blue),
        }
    }

    pub fn black() -> Self {
        Self {
            vec3: vec3!(u8::MIN, u8::MIN, u8::MIN),
        }
    }

    pub fn white() -> Self {
        Self {
            vec3: vec3!(u8::MAX, u8::MAX, u8::MAX),
        }
    }
}
