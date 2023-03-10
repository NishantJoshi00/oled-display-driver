mod loader;

use std::time::Instant;

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    transform::Transform,
};

use loader::disp::get_display;

fn main() {
    let mut display = get_display().unwrap();
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let start = Point::new(0, 0);
    let mut im = Image::new(&raw, start);

    im.draw(&mut *display).unwrap();

    display.flush().unwrap();
    let mut delta = Point::new(1, 0);
    loop {
        let start = Instant::now();
        im.translate_mut(delta.clone());

        match (delta.x, im.bounding_box().top_left.x) {
            (1, x) if x > 64 => delta = Point::new(-1, 0),
            (-1, x) if x < 0 => delta = Point::new(1, 0),
            (_, _) => {}
        }
        display.clear();
        im.draw(&mut *display).unwrap();
        display.flush().unwrap();
        let total = Instant::now() - start;
        println!("Time Taken: {}ms", total.as_millis());
    }
}
