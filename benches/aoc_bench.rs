use std::time::Duration;

use aoc23::{calc_day, Task};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day(day: usize) {
    let mut res1 = String::new();
    let mut res2 = String::new();
    let mut duration = Duration::default();
    calc_day(
        day,
        &mut res1,
        &mut res2,
        &mut duration,
        false,
        Task::Both,
        false,
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| bench_day(black_box(1))));
    c.bench_function("day 2", |b| b.iter(|| bench_day(black_box(2))));
    c.bench_function("day 3", |b| b.iter(|| bench_day(black_box(3))));
    c.bench_function("day 4", |b| b.iter(|| bench_day(black_box(4))));
    c.bench_function("day 5", |b| b.iter(|| bench_day(black_box(5))));
    c.bench_function("day 6", |b| b.iter(|| bench_day(black_box(6))));
    c.bench_function("day 7", |b| b.iter(|| bench_day(black_box(7))));
    c.bench_function("day 8", |b| b.iter(|| bench_day(black_box(8))));
    c.bench_function("day 9", |b| b.iter(|| bench_day(black_box(9))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
