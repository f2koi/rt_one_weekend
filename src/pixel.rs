use gfxmath_vec3::{vec3, Vec3};

#[derive(Clone)]
pub struct Pixel {
    vec3: Vec3<u8>,
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
