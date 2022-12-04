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

        _ => println!("{}: unknown command", command),
    }
}
