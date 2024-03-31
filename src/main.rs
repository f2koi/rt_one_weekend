mod pixel;
mod ppm;

use pixel::Pixel;
use ppm::PPM;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    const RATIO: f32 = 16.0 / 9.0;
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = (WIDTH as f32 / RATIO) as u32;

    let mut image = PPM::new_black(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            image.set_pixel(
                x,
                y,
                Pixel {
                    red: x as u8,
                    green: y as u8,
                    blue: 0,
                },
            );
        }
    }
    let mut file_writer = BufWriter::new(File::create("./output/test.ppm").unwrap());
    image.write_to(&mut file_writer).unwrap();
}
