extern crate image;
extern crate time;

extern crate mandelbrot;
extern crate num;

use std::path::Path;

use mandelbrot::set::Set;
use num::BigRational;

fn main() {
    let over = 20;
    print!(
        "Going to go for {} past the maximum seen escape point\n",
        over
    );
    let set: Set<BigRational> = Set::create(300).iterate_as_required(over);

    let buffer = set.luma_buffer();

    let timestamp = time::get_time().sec;
    let filename = format!("images/{}.png", timestamp);
    image::save_buffer(
        &Path::new(&filename),
        &buffer[..],
        set.size(),
        set.size(),
        image::Gray(8),
    )
    .unwrap();
}
