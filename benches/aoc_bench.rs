use std::time::Duration;

use aoc23::{calc_day, Task};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day(day: usize) {
    let mut res1 = String::new();
    let mut res2 = String::new();
    let mut duration = Duration::default();
    calc_day(day, &mut res1, &mut res2, &mut duration, false, Task::Both);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| bench_day(1)));
    c.bench_function("day 2", |b| b.iter(|| bench_day(2)));
    c.bench_function("day 3", |b| b.iter(|| bench_day(3)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
