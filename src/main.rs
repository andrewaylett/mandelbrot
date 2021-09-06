extern crate image;
extern crate time;

extern crate mandelbrot;
extern crate num;

use std::convert::TryFrom;
use std::path::Path;

use anyhow::{Context, Error};
use structopt::StructOpt;

use mandelbrot::complex::Complex;
use mandelbrot::fix::fix2x61::Fix2x61;
use mandelbrot::set::Set;
use mandelbrot::zoom_path::ZoomPath;

#[derive(StructOpt, Debug)]
#[structopt(name = "mandelbrot")]
struct Args {
    #[structopt(long)]
    path: Option<ZoomPath>,
    #[structopt(short, long)]
    file: Option<String>,
    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let centre = Complex::zero();
    let radius: Fix2x61 = Fix2x61::try_from(2i8)?;

    let mut set: Set = Set::create(8, centre, radius)
        .context("Creating the set")?
        .iterate_as_required(500, args.verbose)?;

    if let Some(path) = args.path {
        for quad in path.0.iter() {
            set = set
                .subset(quad)?
                .iterate_as_required(set.seen_escapes_to(), args.verbose)?;
        }
    }

    let buffer = set.luma_buffer();

    let filename = if let Some(name) = args.file {
        name
    } else {
        let timestamp = time::get_time().sec;
        format!("images/{}.png", timestamp)
    };

    let filename = if filename.ends_with(".png") {
        filename
    } else {
        format!("{}.png", filename)
    };

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
