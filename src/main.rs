extern crate image;
extern crate time;

extern crate mandelbrot;
extern crate num;

use std::path::Path;

use anyhow::{Context, Error};
use mandelbrot::set::Set;

fn main() -> Result<(), Error> {
    let over = 20;
    print!(
        "Going to go for {} past the maximum seen escape point\n",
        over
    );
    let set: Set = Set::create(10)
        .context("Creating the set")?
        .iterate_as_required(over)?;

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
    Ok(())
}
