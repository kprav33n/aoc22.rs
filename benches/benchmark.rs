use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn bench_day01(c: &mut Criterion) {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    c.bench_function("day01::max_total_calories", |b| {
        b.iter(|| aoc22::day01::max_total_calories(&input))
    });
    c.bench_function("day01::max3_total_calories (BinHeap)", |b| {
        b.iter(|| aoc22::day01::max3_total_calories(&input))
    });
}

pub fn bench_day02(c: &mut Criterion) {
    let input = fs::read_to_string("input/day02.txt").unwrap();
    c.bench_function("day02::total_score_p1", |b| {
        b.iter(|| aoc22::day02::total_score_p1(&input))
    });
    c.bench_function("day02::total_score_p2", |b| {
        b.iter(|| aoc22::day02::total_score_p2(&input))
    });
}

pub fn bench_day03(c: &mut Criterion) {
    let input = fs::read_to_string("input/day03.txt").unwrap();
    c.bench_function("day03::sum_priorities_p1", |b| {
        b.iter(|| aoc22::day03::sum_priorities_p1(&input))
    });
    c.bench_function("day03::sum_priorities_p2", |b| {
        b.iter(|| aoc22::day03::sum_priorities_p2(&input))
    });
}

pub fn bench_day04(c: &mut Criterion) {
    let input = fs::read_to_string("input/day04.txt").unwrap();
    c.bench_function("day04::num_fully_contained", |b| {
        b.iter(|| aoc22::day04::num_fully_contained(&input))
    });
    c.bench_function("day04::num_overlapping", |b| {
        b.iter(|| aoc22::day04::num_overlapping(&input))
    });
}

pub fn bench_day05(c: &mut Criterion) {
    let input = fs::read_to_string("input/day05.txt").unwrap();
    c.bench_function("day05::top_of_stack_p1", |b| {
        b.iter(|| aoc22::day05::top_of_stack_p1(&input))
    });
    c.bench_function("day05::top_of_stack_p2", |b| {
        b.iter(|| aoc22::day05::top_of_stack_p2(&input))
    });
}

pub fn bench_day06(c: &mut Criterion) {
    let input = fs::read_to_string("input/day06.txt").unwrap();
    c.bench_function("day06::start_of_packet_p1", |b| {
        b.iter(|| aoc22::day06::start_of_packet_p1(&input))
    });
    c.bench_function("day06::start_of_packet_p2", |b| {
        b.iter(|| aoc22::day06::start_of_packet_p2(&input))
    });
}

criterion_group!(
    benches,
    bench_day01,
    bench_day02,
    bench_day03,
    bench_day04,
    bench_day05,
    bench_day06
);
criterion_main!(benches);
