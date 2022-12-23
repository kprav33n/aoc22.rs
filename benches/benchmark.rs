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

pub fn bench_day07(c: &mut Criterion) {
    let input = fs::read_to_string("input/day07.txt").unwrap();
    c.bench_function("day07::total_size_p1", |b| {
        b.iter(|| aoc22::day07::total_size_p1(&input))
    });
    c.bench_function("day07::total_size_p2", |b| {
        b.iter(|| aoc22::day07::total_size_p2(&input))
    });
}

pub fn bench_day08(c: &mut Criterion) {
    let input = fs::read_to_string("input/day08.txt").unwrap();
    c.bench_function("day08::num_visible_trees", |b| {
        b.iter(|| aoc22::day08::num_visible_trees(&input))
    });
    c.bench_function("day08::highest_scenic_score", |b| {
        b.iter(|| aoc22::day08::highest_scenic_score(&input))
    });
}

pub fn bench_day09(c: &mut Criterion) {
    let input = fs::read_to_string("input/day09.txt").unwrap();
    c.bench_function("day09::num_tail_positions_p1", |b| {
        b.iter(|| aoc22::day09::num_tail_positions_p1(&input))
    });
    c.bench_function("day09::num_tail_positions_p2", |b| {
        b.iter(|| aoc22::day09::num_tail_positions_p2(&input))
    });
}

pub fn bench_day10(c: &mut Criterion) {
    let input = fs::read_to_string("input/day10.txt").unwrap();
    c.bench_function("day10::sum_of_signal_strengths", |b| {
        b.iter(|| aoc22::day10::sum_of_signal_strengths(&input))
    });
    c.bench_function("day10::render_image", |b| {
        b.iter(|| aoc22::day10::render_image(&input))
    });
}

pub fn bench_day11(c: &mut Criterion) {
    let input = fs::read_to_string("input/day11.txt").unwrap();
    c.bench_function("day11::monkey_business_level_p1", |b| {
        b.iter(|| aoc22::day11::monkey_business_level_p1(&input))
    });
    c.bench_function("day11::monkey_business_level_p2", |b| {
        b.iter(|| aoc22::day11::monkey_business_level_p2(&input))
    });
}

pub fn bench_day12(c: &mut Criterion) {
    let input = fs::read_to_string("input/day12.txt").unwrap();
    c.bench_function("day12::num_steps_to_target_p1", |b| {
        b.iter(|| aoc22::day12::num_steps_to_target_p1(&input))
    });
    c.bench_function("day12::num_steps_to_target_p2", |b| {
        b.iter(|| aoc22::day12::num_steps_to_target_p2(&input))
    });
}

pub fn bench_day13(c: &mut Criterion) {
    let input = fs::read_to_string("input/day13.txt").unwrap();
    c.bench_function("day13::sum_right_indices", |b| {
        b.iter(|| aoc22::day13::sum_right_indices(&input))
    });
    c.bench_function("day13::decoder_key", |b| {
        b.iter(|| aoc22::day13::decoder_key(&input))
    });
}

pub fn bench_day14(c: &mut Criterion) {
    let input = fs::read_to_string("input/day14.txt").unwrap();
    c.bench_function("day14::num_resting_sand_units_p1", |b| {
        b.iter(|| aoc22::day14::num_resting_sand_units_p1(&input))
    });
    c.bench_function("day14::num_resting_sand_units_p2", |b| {
        b.iter(|| aoc22::day14::num_resting_sand_units_p2(&input))
    });
}

pub fn bench_day15(c: &mut Criterion) {
    let input = fs::read_to_string("input/day15.txt").unwrap();
    c.bench_function("day15::num_empty_positions", |b| {
        b.iter(|| aoc22::day15::num_empty_positions(&input, 2000000))
    });
    c.bench_function("day15::distress_beacon_tuning_frequency", |b| {
        b.iter(|| aoc22::day15::distress_beacon_tuning_frequency(&input, 4000000))
    });
}

pub fn bench_day18(c: &mut Criterion) {
    let input = fs::read_to_string("input/day18.txt").unwrap();
    c.bench_function("day18::surface_area", |b| {
        b.iter(|| aoc22::day18::surface_area(&input))
    });
    c.bench_function("day18::external_surface_area", |b| {
        b.iter(|| aoc22::day18::external_surface_area(&input))
    });
}

pub fn bench_day20(c: &mut Criterion) {
    let input = fs::read_to_string("input/day20.txt").unwrap();
    c.bench_function("day20::sum_grove_coordinates_p1", |b| {
        b.iter(|| aoc22::day20::sum_grove_coordinates_p1(&input))
    });
    c.bench_function("day20::sum_grove_coordinates_p2", |b| {
        b.iter(|| aoc22::day20::sum_grove_coordinates_p2(&input))
    });
}

pub fn bench_day21(c: &mut Criterion) {
    let input = fs::read_to_string("input/day21.txt").unwrap();
    c.bench_function("day21::root_yells", |b| {
        b.iter(|| aoc22::day21::root_yells(&input))
    });
    c.bench_function("day21::i_yell", |b| b.iter(|| aoc22::day21::i_yell(&input)));
}

pub fn bench_day23(c: &mut Criterion) {
    let input = fs::read_to_string("input/day23.txt").unwrap();
    c.bench_function("day23::empty_ground_tiles", |b| {
        b.iter(|| aoc22::day23::empty_ground_tiles(&input))
    });
    c.bench_function("day23::first_idle_round", |b| {
        b.iter(|| aoc22::day23::first_idle_round(&input))
    });
}

criterion_group!(
    benches,
    bench_day01,
    bench_day02,
    bench_day03,
    bench_day04,
    bench_day05,
    bench_day06,
    bench_day07,
    bench_day08,
    bench_day09,
    bench_day10,
    bench_day11,
    bench_day12,
    bench_day13,
    bench_day14,
    bench_day15,
    bench_day18,
    bench_day20,
    bench_day21,
    bench_day23,
);
criterion_main!(benches);
