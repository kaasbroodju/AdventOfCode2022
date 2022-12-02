use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_one;
mod day_two;
use day_one::*;
use day_two::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut day_one_group = c.benchmark_group("day one");
    day_one_group.bench_function("first part", |b| b.iter(|| day_one::first_part()));
    day_one_group.bench_function("second part", |b| b.iter(|| day_one::second_part()));

    day_one_group.finish();

    let mut day_two_group = c.benchmark_group("day two");
    day_two_group.bench_function("first part", |b| b.iter(|| day_two::first_part()));
    day_two_group.bench_function("second part", |b| b.iter(|| day_two::second_part()));

    day_two_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);