use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
use day_01::*;
use day_02::*;
use day_03::*;
use day_04::*;
use day_05::*;
use day_06::*;
use day_07::*;
use day_08::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    // let mut day_01_group = c.benchmark_group("day 01");
    // day_01_group.bench_function("first part", |b| b.iter(|| day_01::first_part()));
    // day_01_group.bench_function("second part", |b| b.iter(|| day_01::second_part()));
    //
    // day_01_group.finish();
    //
    // let mut day_02_group = c.benchmark_group("day 02");
    // day_02_group.bench_function("first part", |b| b.iter(|| day_02::first_part()));
    // day_02_group.bench_function("second part", |b| b.iter(|| day_02::second_part()));
    //
    // day_02_group.finish();
    //
    // let mut day_03_group = c.benchmark_group("day 03");
    // day_03_group.bench_function("first part", |b| b.iter(|| day_03::first_part()));
    // day_03_group.bench_function("second part", |b| b.iter(|| day_03::second_part()));
    //
    // day_03_group.finish();
    //
    // let mut day_04_group = c.benchmark_group("day 04");
    // day_04_group.bench_function("first part", |b| b.iter(|| day_04::first_part()));
    // day_04_group.bench_function("second part", |b| b.iter(|| day_04::second_part()));
    //
    // day_04_group.finish();
    //
    // let mut day_05_group = c.benchmark_group("day 05");
    // day_05_group.bench_function("first part", |b| b.iter(|| day_05::first_part()));
    // day_05_group.bench_function("second part", |b| b.iter(|| day_05::second_part()));
    //
    // day_05_group.finish();

    // let mut day_06_group = c.benchmark_group("day 06");
    // day_06_group.bench_function("first part", |b| b.iter(|| day_06::first_part()));
    // day_06_group.bench_function("second part", |b| b.iter(|| day_06::second_part()));
    //
    // day_06_group.finish();

    // let mut day_07_group = c.benchmark_group("day 07");
    // day_07_group.bench_function("first part", |b| b.iter(|| day_07::first_part()));
    // day_07_group.bench_function("second part", |b| b.iter(|| day_07::second_part()));
    //
    // day_07_group.finish();

    let mut day_08_group = c.benchmark_group("day 08");
    day_08_group.bench_function("first part", |b| b.iter(|| day_08::first_part()));
    day_08_group.bench_function("second part", |b| b.iter(|| day_08::second_part()));

    day_08_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);