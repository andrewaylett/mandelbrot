use anyhow::{Context, Error};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mandelbrot::complex::{Complex, FixResult};
use mandelbrot::fix::fix2x61::Fix2x61;
use mandelbrot::set::Set;

fn bench_render(c: &mut Criterion) {
    let centre = Complex::new(Fix2x61::zero(), Fix2x61::zero());
    let radius = Fix2x61::two();

    c.bench_function("128x128 over 20", |b| {
        b.iter_with_large_drop(|| -> Result<Set, Error> {
            let mut set =
                Set::create(7, black_box(centre), black_box(radius)).context("Creating the set")?;
            set.iterate_as_required(400, false)?;
            Ok(set)
        })
    });
}

fn bench_iterate(c: &mut Criterion) {
    let zero = Complex::new(Fix2x61::zero(), Fix2x61::zero());
    let mut z1 = black_box(zero);
    let z2 = black_box(zero);

    c.bench_function("iterate zero", |b| {
        b.iter(|| -> FixResult<Complex> { z1.iterate_mandelbrot(&z2).map(|_| z1) })
    });
}

criterion_group!(benches, bench_render, bench_iterate);
criterion_main!(benches);
