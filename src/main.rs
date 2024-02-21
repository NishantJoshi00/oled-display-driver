mod loader;

use std::time::Instant;

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    transform::Transform,
};

use loader::disp::get_display;

fn main() {
    let mut display = get_display().unwrap();
    // let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let start = Point::new(5, 10);
    // let mut im = Image::new(&raw, start);

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);


    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    // create text object
    let text = embedded_graphics::text::Text::new(&buf, start, style);

    text.draw(&mut *display).unwrap();

    display.flush().unwrap();
    // let mut delta = Point::new(1, 0);
    loop {
    //     // let start = Instant::now();
    //     // im.translate_mut(delta.clone());

    //     // match (delta.x, im.bounding_box().top_left.x) {
    //     //     (1, x) if x > 64 => delta = Point::new(-1, 0),
    //     //     (-1, x) if x < 0 => delta = Point::new(1, 0),
    //     //     (_, _) => {}
    //     // }
    //     // display.clear();
    //     // im.draw(&mut *display).unwrap();
    //     // display.flush().unwrap();
    //     // let total = Instant::now() - start;
    //     // println!("Time Taken: {}ms", total.as_millis());
    }
}
