use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_01;
mod day_02;
use day_01::*;
use day_02::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut day_one_group = c.benchmark_group("day one");
    day_one_group.bench_function("first part", |b| b.iter(|| day_01::first_part()));
    day_one_group.bench_function("second part", |b| b.iter(|| day_01::second_part()));

    day_one_group.finish();

    let mut day_two_group = c.benchmark_group("day two");
    day_two_group.bench_function("first part", |b| b.iter(|| day_02::first_part()));
    day_two_group.bench_function("second part", |b| b.iter(|| day_02::second_part()));

    day_two_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);