use aoc22::day01::{max3_total_calories, max_total_calories};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn bench_day01(c: &mut Criterion) {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("max_total_calories", |b| {
        b.iter(|| max_total_calories(&input))
    });
    c.bench_function("max3_total_calories (BinHeap)", |b| {
        b.iter(|| max3_total_calories(&input))
    });
}

criterion_group!(benches, bench_day01);
criterion_main!(benches);
