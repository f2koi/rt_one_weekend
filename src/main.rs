use std::fs::File;
use std::io::BufWriter;

#[derive(Copy, Clone)]
struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn black() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }
}


struct PPM {
    width: u32,
    height: u32,
    data: Vec<Pixel>,
}

impl PPM {
    pub fn new_black(width: u32, height: u32) -> Self {
        let data = vec![Pixel::black(); width as usize * height as usize];
        Self { width, height, data }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.data[self.width as usize * y as usize + x as usize] = pixel;
    }

    fn write_to(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;
        for pixel in &self.data {
            writeln!(writer, "{} {} {}", pixel.red, pixel.green, pixel.blue)?;
        }
        Ok(())
    }
}

fn main() {
    const RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = (WIDTH as f32 / RATIO) as u32;

    let mut image = PPM::new_black(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            image.set_pixel(x, y, Pixel { red: x as u8, green: y as u8, blue: 0 });
        }
    }
    let mut file_writer = BufWriter::new(File::create("./output/test.ppm").unwrap());
    image.write_to(&mut file_writer).unwrap();
}
