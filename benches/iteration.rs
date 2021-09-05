use anyhow::Error;
use iai::{black_box, main};

use mandelbrot::complex::Complex;
use mandelbrot::fix::fix2x61::Fix2x61;
use mandelbrot::point::Point;
use mandelbrot::set::Set;

fn iai_benchmark_zero_five_times() -> Result<Point, Error> {
    let p: Point = black_box(Point::ORIGIN);
    p.iterate_n(5)
}

fn iai_benchmark_zero() -> Result<Point, Error> {
    let mut p: Point = black_box(Point::ORIGIN);
    p.iterate()?;
    Ok(p)
}

fn iai_benchmark_full_set_render() -> Result<Set, Error> {
    let centre = Complex::new(Fix2x61::zero(), Fix2x61::zero());
    let radius = Fix2x61::two();

    Set::create(7, black_box(centre), black_box(radius))?.iterate_as_required(1000, 20, false)
}

main!(
    iai_benchmark_zero,
    iai_benchmark_zero_five_times,
    iai_benchmark_full_set_render
);
