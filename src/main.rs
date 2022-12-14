use std::env;
use std::io;
use std::process;

fn main() {
    let Some(command) = env::args().nth(1) else {
        println!("Usage: aoc21 <dayNNx>");
        process::exit(1);
    };

    let Ok(input) = io::read_to_string(io::stdin()) else {
        println!("Failed to read input from stdin");
        process::exit(2);
    };

    match command.as_ref() {
        "day01a" => println!("{}", aoc22::day01::max_total_calories(&input)),
        "day01b" => println!("{}", aoc22::day01::max3_total_calories(&input)),
        "day02a" => println!("{}", aoc22::day02::total_score_p1(&input)),
        "day02b" => println!("{}", aoc22::day02::total_score_p2(&input)),
        "day03a" => println!("{}", aoc22::day03::sum_priorities_p1(&input)),
        "day03b" => println!("{}", aoc22::day03::sum_priorities_p2(&input)),
        "day04a" => println!("{}", aoc22::day04::num_fully_contained(&input)),
        "day04b" => println!("{}", aoc22::day04::num_overlapping(&input)),
        "day05a" => println!("{}", aoc22::day05::top_of_stack_p1(&input)),
        "day05b" => println!("{}", aoc22::day05::top_of_stack_p2(&input)),
        "day06a" => println!("{}", aoc22::day06::start_of_packet_p1(&input)),
        "day06b" => println!("{}", aoc22::day06::start_of_packet_p2(&input)),
        "day07a" => println!("{}", aoc22::day07::total_size_p1(&input)),
        "day07b" => println!("{}", aoc22::day07::total_size_p2(&input)),
        "day08a" => println!("{}", aoc22::day08::num_visible_trees(&input)),
        "day08b" => println!("{}", aoc22::day08::highest_scenic_score(&input)),
        "day09a" => println!("{}", aoc22::day09::num_tail_positions_p1(&input)),
        "day09b" => println!("{}", aoc22::day09::num_tail_positions_p2(&input)),
        "day10a" => println!("{}", aoc22::day10::sum_of_signal_strengths(&input)),
        "day10b" => println!("{}", aoc22::day10::render_image(&input)),
        "day11a" => println!("{}", aoc22::day11::monkey_business_level_p1(&input)),
        "day11b" => println!("{}", aoc22::day11::monkey_business_level_p2(&input)),
        "day12a" => println!("{}", aoc22::day12::num_steps_to_target_p1(&input)),
        "day12b" => println!("{}", aoc22::day12::num_steps_to_target_p2(&input)),
        "day13a" => println!("{}", aoc22::day13::sum_right_indices(&input)),
        "day13b" => println!("{}", aoc22::day13::decoder_key(&input)),
        "day14a" => println!("{}", aoc22::day14::num_resting_sand_units_p1(&input)),
        "day14b" => println!("{}", aoc22::day14::num_resting_sand_units_p2(&input)),

        _ => println!("{}: unknown command", command),
    }
}
