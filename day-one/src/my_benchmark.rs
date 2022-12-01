use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod day_one;
use day_one::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut day_one_group = c.benchmark_group("day one");
    day_one_group.bench_function("first part", |b| b.iter(|| first_part()));
    day_one_group.bench_function("second part", |b| b.iter(|| second_part()));

    // c.bench_function("fib 20", |b| b.iter(|| first_part()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);