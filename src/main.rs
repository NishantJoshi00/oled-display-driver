mod loader;

use embedded_graphics::{
    transform::Transform,
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};

use loader::disp::get_display;

fn main() {
    let mut display = get_display().unwrap();
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let start = Point::new(0,0);
    let mut im = Image::new(&raw, start);

    im.draw(&mut *display).unwrap();

    display.flush().unwrap();
    let mut delta = Point::new(1, 0);
    loop {
        im.translate_mut(delta.clone());

        match (delta.x, im.bounding_box().top_left.x) {
            (1, x) if x > 64 => { delta = Point::new(-1, 0) }
            (-1, x) if x < 0 => { delta = Point::new(1, 0) }
            (_, _) => {}
        }
        display.clear();
        im.draw(&mut *display).unwrap();
        display.flush().unwrap();
    }
}
