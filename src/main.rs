use std::fs::File;
use std::io::BufWriter;

use pixel::Pixel;
use ppm::PPM;

mod pixel;
mod ppm;

fn normalize(x: usize, interval: (usize, usize)) -> f32 {
    let (left, right) = interval;
    assert!(left <= x && x < right);
    let (numerator, denominator) = (x - left, right - left);
    numerator as f32 / denominator as f32
}

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
                Pixel::new(
                    (normalize(x as usize, (0, WIDTH as usize)) * 256.0) as u8,
                    (normalize(y as usize, (0, HEIGHT as usize)) * 256.0) as u8,
                    0,
                ),
            );
        }
    }

    let mut file_writer = BufWriter::new(File::create("./test.ppm").unwrap());
    image.write_to(&mut file_writer).unwrap();
}
