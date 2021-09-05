use anyhow::{Context, Error};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandelbrot::complex::{Complex, Fix2x61, FixResult};
use mandelbrot::set::Set;

fn bench_render(c: &mut Criterion) {
    let centre = Complex::new(Fix2x61::zero(), Fix2x61::zero());
    let radius = Fix2x61::two();

    c.bench_function("128x128 over 20", |b| {
        b.iter_with_large_drop(|| -> Result<Set, Error> {
            Set::create(7, black_box(centre), black_box(radius))
                .context("Creating the set")?
                .iterate_as_required(20)
        })
    });
}

fn bench_iterate(c: &mut Criterion) {
    let zero = Complex::new(Fix2x61::zero(), Fix2x61::zero());

    c.bench_function("iterate zero", |b| {
        b.iter(|| -> FixResult<Complex> { black_box(zero).iterate_mandelbrot(&black_box(zero)) })
    });
}

criterion_group!(benches, bench_render, bench_iterate);
criterion_main!(benches);
