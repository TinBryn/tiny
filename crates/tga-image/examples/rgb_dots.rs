use tga_image::{Color, Image};

const RED: Color = Color::rgb(255, 0, 0);
const GREEN: Color = Color::rgb(0, 255, 0);
const BLUE: Color = Color::rgb(0, 0, 255);

fn main() {
    let mut image = Image::new(100, 100, 3);
    image.set(52, 41, RED);
    image.set(54, 41, GREEN);
    image.set(56, 41, BLUE);
    image.flip_vertically();
    image.write_tga_file("rgb_dots.tga", false).unwrap();
}
