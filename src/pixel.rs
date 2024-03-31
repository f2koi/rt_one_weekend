#[derive(Copy, Clone)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn black() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
}
