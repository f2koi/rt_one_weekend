use crate::pixel::Pixel;

pub struct PPM {
    width: u32,
    height: u32,
    data: Vec<Pixel>,
}

impl PPM {
    pub fn new_black(width: u32, height: u32) -> Self {
        let data = vec![Pixel::black(); width as usize * height as usize];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.data[self.width as usize * y as usize + x as usize] = pixel;
    }

    pub fn write_to(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;
        for pixel in &self.data {
            writeln!(writer, "{} {} {}", pixel.red, pixel.green, pixel.blue)?;
        }
        Ok(())
    }
}
