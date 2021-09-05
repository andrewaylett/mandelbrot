use iai::{black_box, main};
use mandelbrot::complex::{Complex, Fix2x61, FixResult};

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

main!(iai_benchmark_zero, iai_benchmark_zero_five_times);
