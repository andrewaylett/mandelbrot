use anyhow::Error;
use iai::{black_box, main};
use mandelbrot::complex::{Complex, Fix2x61, FixResult};
use mandelbrot::set::Set;

const ZERO: Complex = Complex::new(Fix2x61::zero(), Fix2x61::zero());

fn iai_benchmark_zero_five_times() -> FixResult<Complex> {
    let z = black_box(ZERO);
    black_box(ZERO)
        .iterate_mandelbrot(&z)?
        .iterate_mandelbrot(&z)?
        .iterate_mandelbrot(&z)?
        .iterate_mandelbrot(&z)?
        .iterate_mandelbrot(&z)
}

fn iai_benchmark_zero() -> FixResult<Complex> {
    black_box(ZERO).iterate_mandelbrot(&black_box(ZERO))
}

fn iai_benchmark_full_set_render() -> Result<Set, Error> {
    let centre = Complex::new(Fix2x61::zero(), Fix2x61::zero());
    let radius = Fix2x61::two();

    Set::create(7, black_box(centre), black_box(radius))?.iterate_as_required(20)
}

main!(
    iai_benchmark_zero,
    iai_benchmark_zero_five_times,
    iai_benchmark_full_set_render
);
