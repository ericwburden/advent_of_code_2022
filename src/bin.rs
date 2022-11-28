// Declare modules for each day here
use aoc2022lib::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The day to run
    #[arg(default_value_t = 1)]
    day: u8,
}

fn main() {
    let day = Args::parse().day;

    let run = match day {
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        9 => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        21 => day21::run,
        22 => day22::run,
        23 => day23::run,
        24 => day24::run,
        25 => day25::run,
        _ => panic!("There's no day {} on the Advent Calendar!", day),
    };

    let answer_one = run(Part::One);
    let answer_two = run(Part::Two);

    println!("************************************************************");
    println!("* Advent of Code: 2022");
    println!("*   Solution for...");
    println!("*     Part One: {}", answer_one);
    println!("*     Part Two: {}", answer_two);
    println!("************************************************************");
}
