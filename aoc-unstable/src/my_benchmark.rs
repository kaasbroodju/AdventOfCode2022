#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_02;
use day_02::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut day_02_group = c.benchmark_group("day 02");
    day_02_group.bench_function("first part", |b| b.iter(|| day_02::first_part()));
    day_02_group.bench_function("second part", |b| b.iter(|| day_02::second_part()));

    day_02_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);