extern crate image;
extern crate time;

extern crate mandelbrot;
extern crate num;

use std::convert::TryInto;
use std::path::Path;

use anyhow::{Context, Error};
use structopt::StructOpt;

use mandelbrot::complex::Complex;
use mandelbrot::fix::fix2x61::Fix2x61;
use mandelbrot::set::Set;

#[derive(StructOpt, Debug)]
#[structopt(name = "mandelbrot")]
struct Args {
    #[structopt(default_value = "20", short, long)]
    over: u64,
    #[structopt(default_value = "0,0", short, long)]
    centre: Complex,
    #[structopt(default_value = "2", short, long)]
    radius: f64,
    #[structopt(default_value = "7", short, long)]
    size: usize,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();
    let over = args.over;
    println!(
        "Going to go for {} past the maximum seen escape point",
        over
    );

    let centre = args.centre;
    let radius: Fix2x61 = args.radius.try_into()?;

    let set: Set = Set::create(args.size, centre, radius)
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
