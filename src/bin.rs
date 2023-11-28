// Declare modules for each day here
use aoc2022lib::*;
use clap::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run one specific day
    #[arg(short, long)]
    day: Option<u8>,

    /// Run all days
    #[arg(short, long)]
    all: bool,

    /// Run with timing
    #[arg(short, long)]
    timed: bool,
}

fn main() {
    let args = Args::parse();
    let all = args.all;
    let timed = args.timed;
    let timer = Instant::now();

    if all {
        if timed {
            let mut results = Vec::with_capacity(25);
            for day in 1..=25 {
                results.push(run_timed(day, &timer));
            }
            let total = timer.elapsed();
            results.iter().for_each(|result| println!("{}", result));
            println!("Total Runtime: {total:?}");
        } else {
            (1..=25).for_each(|day| println!("{}", run_day(day)));
        }
    } else if let Some(day) = args.day {
        if timed {
            println!("{}", run_timed(day, &timer));
        } else {
            println!("{}", run_day(day));
        }
    } else {
        println!("Didn't do anything. Run with --help to see flags.")
    }
}

struct RunResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let RunResult {
            day,
            answer_one,
            answer_two,
        } = self;
        writeln!(
            f,
            "************************************************************"
        )?;
        writeln!(f, "* Advent of Code: 2022, Day {day}")?;
        writeln!(f, "*   Solution for...")?;
        writeln!(f, "*     Part One: {answer_one}")?;
        writeln!(f, "*     Part Two: {answer_two}")?;
        writeln!(
            f,
            "************************************************************"
        )?;
        write!(f, "")
    }
}

fn run_day(day: u8) -> RunResult {
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
        _ => panic!("There's no day {day} on the Advent Calendar!"),
    };

    let answer_one = run(Part::One);
    let answer_two = run(Part::Two);
    RunResult {
        day,
        answer_one,
        answer_two,
    }
}

struct TimedResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
    duration: Duration,
}

impl Display for TimedResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let TimedResult {
            day,
            answer_one,
            answer_two,
            duration,
        } = self;
        writeln!(
            f,
            "************************************************************"
        )?;
        writeln!(f, "* Advent of Code: 2022, Day {day}")?;
        writeln!(f, "*   Solution for...")?;
        writeln!(f, "*     Part One: {answer_one}")?;
        writeln!(f, "*     Part Two: {answer_two}")?;
        writeln!(f, "* Run Time: {duration:?}")?;
        writeln!(
            f,
            "************************************************************"
        )?;
        write!(f, "")
    }
}

fn run_timed(day: u8, timer: &Instant) -> TimedResult {
    let run = match day {
        1 => day01::run_both,
        2 => day02::run_both,
        3 => day03::run_both,
        4 => day04::run_both,
        5 => day05::run_both,
        6 => day06::run_both,
        7 => day07::run_both,
        8 => day08::run_both,
        9 => day09::run_both,
        10 => day10::run_both,
        11 => day11::run_both,
        12 => day12::run_both,
        13 => day13::run_both,
        14 => day14::run_both,
        15 => day15::run_both,
        16 => day16::run_both,
        17 => day17::run_both,
        18 => day18::run_both,
        19 => day19::run_both,
        20 => day20::run_both,
        21 => day21::run_both,
        22 => day22::run_both,
        23 => day23::run_both,
        24 => day24::run_both,
        25 => day25::run_both,
        _ => panic!("There's no day {day} on the Advent Calendar!"),
    };

    let start = timer.elapsed();
    let (answer_one, answer_two) = run();
    let duration = timer.elapsed() - start;
    TimedResult {
        day,
        answer_one,
        answer_two,
        duration,
    }
}
