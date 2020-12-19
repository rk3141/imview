extern crate image;

use crate::image::GenericImageView;

fn main() {
    let arg: Vec<String> = std::env::args().collect::<Vec<String>>();
    if arg.len() == 1 {
        return;
    }
    let img = image::open(arg[1].clone()).unwrap();
    let (width, _) = img.dimensions();

    for (i, (_, _, p)) in img.pixels().enumerate() {
        pallete::printbg(" ", pallete::Color::Rgb(p.0[0], p.0[1], p.0[2])).unwrap();
        if i % width as usize == width as usize - 1 {
            print!("\n");
        }
    }
}
