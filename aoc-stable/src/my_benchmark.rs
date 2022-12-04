use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_01;
mod day_02;
mod day_03;
mod day_04;
use day_01::*;
use day_02::*;
use day_03::*;
use day_04::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut day_01_group = c.benchmark_group("day 01");
    day_01_group.bench_function("first part", |b| b.iter(|| day_01::first_part()));
    day_01_group.bench_function("second part", |b| b.iter(|| day_01::second_part()));

    day_01_group.finish();

    let mut day_02_group = c.benchmark_group("day 02");
    day_02_group.bench_function("first part", |b| b.iter(|| day_02::first_part()));
    day_02_group.bench_function("second part", |b| b.iter(|| day_02::second_part()));

    day_02_group.finish();

    let mut day_03_group = c.benchmark_group("day 03");
    day_03_group.bench_function("first part", |b| b.iter(|| day_03::first_part()));
    day_03_group.bench_function("second part", |b| b.iter(|| day_03::second_part()));

    day_03_group.finish();

    let mut day_04_group = c.benchmark_group("day 04");
    day_04_group.bench_function("first part", |b| b.iter(|| day_04::first_part()));
    day_04_group.bench_function("second part", |b| b.iter(|| day_04::second_part()));

    day_04_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);