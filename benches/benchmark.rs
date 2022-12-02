use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn bench_day01(c: &mut Criterion) {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("max_total_calories", |b| {
        b.iter(|| aoc22::day01::max_total_calories(&input))
    });
    c.bench_function("max3_total_calories (BinHeap)", |b| {
        b.iter(|| aoc22::day01::max3_total_calories(&input))
    });
}

pub fn bench_day02(c: &mut Criterion) {
    let input = fs::read_to_string("input/day02.txt").unwrap();
    c.bench_function("total_score_p1", |b| {
        b.iter(|| aoc22::day02::total_score_p1(&input))
    });
    c.bench_function("total_score_p2", |b| {
        b.iter(|| aoc22::day02::total_score_p2(&input))
    });
}

criterion_group!(benches, bench_day01, bench_day02);
criterion_main!(benches);
