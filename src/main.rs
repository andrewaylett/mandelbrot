extern crate image;
extern crate time;

extern crate mandelbrot;

use std::path::Path;

use mandelbrot::set::Set;

fn main() {
    let set = Set::create(300).iterate_to(1000);

    let buffer = set.luma_buffer();
    
    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(&Path::new(&filename),
        &buffer[..],
        set.size(),
        set.size(),
        image::Gray(8)).unwrap();
}
