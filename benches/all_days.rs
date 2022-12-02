use aoc2022lib::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

//-------------------------------------------------------------------------------------
//Day 01-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day01_input(c: &mut Criterion) {
    c.bench_function("Day 01, Parse Input", |b| b.iter(day01::input::read));
}

pub fn benchmark_day01_part01(c: &mut Criterion) {
    let input = black_box(day01::input::read());
    c.bench_function("Day 01, Part 1", |b| b.iter(|| day01::part1::solve(&input)));
}

pub fn benchmark_day01_part02(c: &mut Criterion) {
    let input = black_box(day01::input::read());
    c.bench_function("Day 01, Part 2", |b| b.iter(|| day01::part2::solve(&input)));
}

criterion_group!(
    day01,
    benchmark_day01_input,
    benchmark_day01_part01,
    benchmark_day01_part02
);

//-------------------------------------------------------------------------------------
//Day 02-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day02_input(c: &mut Criterion) {
    c.bench_function("Day 02, Parse Input", |b| b.iter(day02::input::read));
}

pub fn benchmark_day02_part01(c: &mut Criterion) {
    let input = black_box(day02::input::read());
    c.bench_function("Day 02, Part 1", |b| b.iter(|| day02::part1::solve(&input)));
}

pub fn benchmark_day02_part02(c: &mut Criterion) {
    let input = black_box(day02::input::read());
    c.bench_function("Day 02, Part 2", |b| b.iter(|| day02::part2::solve(&input)));
}

criterion_group!(
    day02,
    benchmark_day02_input,
    benchmark_day02_part01,
    benchmark_day02_part02
);

//-------------------------------------------------------------------------------------
//Day 03-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day03_input(c: &mut Criterion) {
    c.bench_function("Day 03, Parse Input", |b| b.iter(day03::input::read));
}

pub fn benchmark_day03_part01(c: &mut Criterion) {
    let input = black_box(day03::input::read());
    c.bench_function("Day 03, Part 1", |b| b.iter(|| day03::part1::solve(&input)));
}

pub fn benchmark_day03_part02(c: &mut Criterion) {
    let input = black_box(day03::input::read());
    c.bench_function("Day 03, Part 2", |b| b.iter(|| day03::part2::solve(&input)));
}

criterion_group!(
    day03,
    benchmark_day03_input,
    benchmark_day03_part01,
    benchmark_day03_part02
);

//-------------------------------------------------------------------------------------
//Day 04-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day04_input(c: &mut Criterion) {
    c.bench_function("Day 04, Parse Input", |b| b.iter(day04::input::read));
}

pub fn benchmark_day04_part01(c: &mut Criterion) {
    let input = black_box(day04::input::read());
    c.bench_function("Day 04, Part 1", |b| b.iter(|| day04::part1::solve(&input)));
}

pub fn benchmark_day04_part02(c: &mut Criterion) {
    let input = black_box(day04::input::read());
    c.bench_function("Day 04, Part 2", |b| b.iter(|| day04::part2::solve(&input)));
}

criterion_group!(
    day04,
    benchmark_day04_input,
    benchmark_day04_part01,
    benchmark_day04_part02
);

//-------------------------------------------------------------------------------------
//Day 05-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day05_input(c: &mut Criterion) {
    c.bench_function("Day 05, Parse Input", |b| b.iter(day05::input::read));
}

pub fn benchmark_day05_part01(c: &mut Criterion) {
    let input = black_box(day05::input::read());
    c.bench_function("Day 05, Part 1", |b| b.iter(|| day05::part1::solve(&input)));
}

pub fn benchmark_day05_part02(c: &mut Criterion) {
    let input = black_box(day05::input::read());
    c.bench_function("Day 05, Part 2", |b| b.iter(|| day05::part2::solve(&input)));
}

criterion_group!(
    day05,
    benchmark_day05_input,
    benchmark_day05_part01,
    benchmark_day05_part02
);

//-------------------------------------------------------------------------------------
//Day 06-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day06_input(c: &mut Criterion) {
    c.bench_function("Day 06, Parse Input", |b| b.iter(day06::input::read));
}

pub fn benchmark_day06_part01(c: &mut Criterion) {
    let input = black_box(day06::input::read());
    c.bench_function("Day 06, Part 1", |b| b.iter(|| day06::part1::solve(&input)));
}

pub fn benchmark_day06_part02(c: &mut Criterion) {
    let input = black_box(day06::input::read());
    c.bench_function("Day 06, Part 2", |b| b.iter(|| day06::part2::solve(&input)));
}

criterion_group!(
    day06,
    benchmark_day06_input,
    benchmark_day06_part01,
    benchmark_day06_part02
);

//-------------------------------------------------------------------------------------
//Day 07-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day07_input(c: &mut Criterion) {
    c.bench_function("Day 07, Parse Input", |b| b.iter(day07::input::read));
}

pub fn benchmark_day07_part01(c: &mut Criterion) {
    let input = black_box(day07::input::read());
    c.bench_function("Day 07, Part 1", |b| b.iter(|| day07::part1::solve(&input)));
}

pub fn benchmark_day07_part02(c: &mut Criterion) {
    let input = black_box(day07::input::read());
    c.bench_function("Day 07, Part 2", |b| b.iter(|| day07::part2::solve(&input)));
}

criterion_group!(
    day07,
    benchmark_day07_input,
    benchmark_day07_part01,
    benchmark_day07_part02
);

//-------------------------------------------------------------------------------------
//Day 08-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day08_input(c: &mut Criterion) {
    c.bench_function("Day 08, Parse Input", |b| b.iter(day08::input::read));
}

pub fn benchmark_day08_part01(c: &mut Criterion) {
    let input = black_box(day08::input::read());
    c.bench_function("Day 08, Part 1", |b| b.iter(|| day08::part1::solve(&input)));
}

pub fn benchmark_day08_part02(c: &mut Criterion) {
    let input = black_box(day08::input::read());
    c.bench_function("Day 08, Part 2", |b| b.iter(|| day08::part2::solve(&input)));
}

criterion_group!(
    day08,
    benchmark_day08_input,
    benchmark_day08_part01,
    benchmark_day08_part02
);

//-------------------------------------------------------------------------------------
//Day 09-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day09_input(c: &mut Criterion) {
    c.bench_function("Day 09, Parse Input", |b| b.iter(day09::input::read));
}

pub fn benchmark_day09_part01(c: &mut Criterion) {
    let input = black_box(day09::input::read());
    c.bench_function("Day 09, Part 1", |b| b.iter(|| day09::part1::solve(&input)));
}

pub fn benchmark_day09_part02(c: &mut Criterion) {
    let input = black_box(day09::input::read());
    c.bench_function("Day 09, Part 2", |b| b.iter(|| day09::part2::solve(&input)));
}

criterion_group!(
    day09,
    benchmark_day09_input,
    benchmark_day09_part01,
    benchmark_day09_part02
);

//-------------------------------------------------------------------------------------
//Day 10-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day10_input(c: &mut Criterion) {
    c.bench_function("Day 10, Parse Input", |b| b.iter(day10::input::read));
}

pub fn benchmark_day10_part01(c: &mut Criterion) {
    let input = black_box(day10::input::read());
    c.bench_function("Day 10, Part 1", |b| b.iter(|| day10::part1::solve(&input)));
}

pub fn benchmark_day10_part02(c: &mut Criterion) {
    let input = black_box(day10::input::read());
    c.bench_function("Day 10, Part 2", |b| b.iter(|| day10::part2::solve(&input)));
}

criterion_group!(
    day10,
    benchmark_day10_input,
    benchmark_day10_part01,
    benchmark_day10_part02
);

//-------------------------------------------------------------------------------------
//Day 11-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day11_input(c: &mut Criterion) {
    c.bench_function("Day 11, Parse Input", |b| b.iter(day11::input::read));
}

pub fn benchmark_day11_part01(c: &mut Criterion) {
    let input = black_box(day11::input::read());
    c.bench_function("Day 11, Part 1", |b| b.iter(|| day11::part1::solve(&input)));
}

pub fn benchmark_day11_part02(c: &mut Criterion) {
    let input = black_box(day11::input::read());
    c.bench_function("Day 11, Part 2", |b| b.iter(|| day11::part2::solve(&input)));
}

criterion_group!(
    day11,
    benchmark_day11_input,
    benchmark_day11_part01,
    benchmark_day11_part02
);

//-------------------------------------------------------------------------------------
//Day 12-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day12_input(c: &mut Criterion) {
    c.bench_function("Day 12, Parse Input", |b| b.iter(day12::input::read));
}

pub fn benchmark_day12_part01(c: &mut Criterion) {
    let input = black_box(day12::input::read());
    c.bench_function("Day 12, Part 1", |b| b.iter(|| day12::part1::solve(&input)));
}

pub fn benchmark_day12_part02(c: &mut Criterion) {
    let input = black_box(day12::input::read());
    c.bench_function("Day 12, Part 2", |b| b.iter(|| day12::part2::solve(&input)));
}

criterion_group!(
    day12,
    benchmark_day12_input,
    benchmark_day12_part01,
    benchmark_day12_part02
);

//-------------------------------------------------------------------------------------
//Day 13-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day13_input(c: &mut Criterion) {
    c.bench_function("Day 13, Parse Input", |b| b.iter(day13::input::read));
}

pub fn benchmark_day13_part01(c: &mut Criterion) {
    let input = black_box(day13::input::read());
    c.bench_function("Day 13, Part 1", |b| b.iter(|| day13::part1::solve(&input)));
}

pub fn benchmark_day13_part02(c: &mut Criterion) {
    let input = black_box(day13::input::read());
    c.bench_function("Day 13, Part 2", |b| b.iter(|| day13::part2::solve(&input)));
}

criterion_group!(
    day13,
    benchmark_day13_input,
    benchmark_day13_part01,
    benchmark_day13_part02
);

//-------------------------------------------------------------------------------------
//Day 14-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day14_input(c: &mut Criterion) {
    c.bench_function("Day 14, Parse Input", |b| b.iter(day14::input::read));
}

pub fn benchmark_day14_part01(c: &mut Criterion) {
    let input = black_box(day14::input::read());
    c.bench_function("Day 14, Part 1", |b| b.iter(|| day14::part1::solve(&input)));
}

pub fn benchmark_day14_part02(c: &mut Criterion) {
    let input = black_box(day14::input::read());
    c.bench_function("Day 14, Part 2", |b| b.iter(|| day14::part2::solve(&input)));
}

criterion_group!(
    day14,
    benchmark_day14_input,
    benchmark_day14_part01,
    benchmark_day14_part02
);

//-------------------------------------------------------------------------------------
//Day 15-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day15_input(c: &mut Criterion) {
    c.bench_function("Day 15, Parse Input", |b| b.iter(day15::input::read));
}

pub fn benchmark_day15_part01(c: &mut Criterion) {
    let input = black_box(day15::input::read());
    c.bench_function("Day 15, Part 1", |b| b.iter(|| day15::part1::solve(&input)));
}

pub fn benchmark_day15_part02(c: &mut Criterion) {
    let input = black_box(day15::input::read());
    c.bench_function("Day 15, Part 2", |b| b.iter(|| day15::part2::solve(&input)));
}

criterion_group!(
    day15,
    benchmark_day15_input,
    benchmark_day15_part01,
    benchmark_day15_part02
);

//-------------------------------------------------------------------------------------
//Day 16-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day16_input(c: &mut Criterion) {
    c.bench_function("Day 16, Parse Input", |b| b.iter(day16::input::read));
}

pub fn benchmark_day16_part01(c: &mut Criterion) {
    let input = black_box(day16::input::read());
    c.bench_function("Day 16, Part 1", |b| b.iter(|| day16::part1::solve(&input)));
}

pub fn benchmark_day16_part02(c: &mut Criterion) {
    let input = black_box(day16::input::read());
    c.bench_function("Day 16, Part 2", |b| b.iter(|| day16::part2::solve(&input)));
}

criterion_group!(
    day16,
    benchmark_day16_input,
    benchmark_day16_part01,
    benchmark_day16_part02
);

//-------------------------------------------------------------------------------------
//Day 17-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day17_input(c: &mut Criterion) {
    c.bench_function("Day 17, Parse Input", |b| b.iter(day17::input::read));
}

pub fn benchmark_day17_part01(c: &mut Criterion) {
    let input = black_box(day17::input::read());
    c.bench_function("Day 17, Part 1", |b| b.iter(|| day17::part1::solve(&input)));
}

pub fn benchmark_day17_part02(c: &mut Criterion) {
    let input = black_box(day17::input::read());
    c.bench_function("Day 17, Part 2", |b| b.iter(|| day17::part2::solve(&input)));
}

criterion_group!(
    day17,
    benchmark_day17_input,
    benchmark_day17_part01,
    benchmark_day17_part02
);

//-------------------------------------------------------------------------------------
//Day 18-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day18_input(c: &mut Criterion) {
    c.bench_function("Day 18, Parse Input", |b| b.iter(day18::input::read));
}

pub fn benchmark_day18_part01(c: &mut Criterion) {
    let input = black_box(day18::input::read());
    c.bench_function("Day 18, Part 1", |b| b.iter(|| day18::part1::solve(&input)));
}

pub fn benchmark_day18_part02(c: &mut Criterion) {
    let input = black_box(day18::input::read());
    c.bench_function("Day 18, Part 2", |b| b.iter(|| day18::part2::solve(&input)));
}

criterion_group!(
    day18,
    benchmark_day18_input,
    benchmark_day18_part01,
    benchmark_day18_part02
);

//-------------------------------------------------------------------------------------
//Day 19-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day19_input(c: &mut Criterion) {
    c.bench_function("Day 19, Parse Input", |b| b.iter(day19::input::read));
}

pub fn benchmark_day19_part01(c: &mut Criterion) {
    let input = black_box(day19::input::read());
    c.bench_function("Day 19, Part 1", |b| b.iter(|| day19::part1::solve(&input)));
}

pub fn benchmark_day19_part02(c: &mut Criterion) {
    let input = black_box(day19::input::read());
    c.bench_function("Day 19, Part 2", |b| b.iter(|| day19::part2::solve(&input)));
}

criterion_group!(
    day19,
    benchmark_day19_input,
    benchmark_day19_part01,
    benchmark_day19_part02
);

//-------------------------------------------------------------------------------------
//Day 20-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day20_input(c: &mut Criterion) {
    c.bench_function("Day 20, Parse Input", |b| b.iter(day20::input::read));
}

pub fn benchmark_day20_part01(c: &mut Criterion) {
    let input = black_box(day20::input::read());
    c.bench_function("Day 20, Part 1", |b| b.iter(|| day20::part1::solve(&input)));
}

pub fn benchmark_day20_part02(c: &mut Criterion) {
    let input = black_box(day20::input::read());
    c.bench_function("Day 20, Part 2", |b| b.iter(|| day20::part2::solve(&input)));
}

criterion_group!(
    day20,
    benchmark_day20_input,
    benchmark_day20_part01,
    benchmark_day20_part02
);

//-------------------------------------------------------------------------------------
//Day 21-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day21_input(c: &mut Criterion) {
    c.bench_function("Day 21, Parse Input", |b| b.iter(day21::input::read));
}

pub fn benchmark_day21_part01(c: &mut Criterion) {
    let input = black_box(day21::input::read());
    c.bench_function("Day 21, Part 1", |b| b.iter(|| day21::part1::solve(&input)));
}

pub fn benchmark_day21_part02(c: &mut Criterion) {
    let input = black_box(day21::input::read());
    c.bench_function("Day 21, Part 2", |b| b.iter(|| day21::part2::solve(&input)));
}

criterion_group!(
    day21,
    benchmark_day21_input,
    benchmark_day21_part01,
    benchmark_day21_part02
);

//-------------------------------------------------------------------------------------
//Day 22-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day22_input(c: &mut Criterion) {
    c.bench_function("Day 22, Parse Input", |b| b.iter(day22::input::read));
}

pub fn benchmark_day22_part01(c: &mut Criterion) {
    let input = black_box(day22::input::read());
    c.bench_function("Day 22, Part 1", |b| b.iter(|| day22::part1::solve(&input)));
}

pub fn benchmark_day22_part02(c: &mut Criterion) {
    let input = black_box(day22::input::read());
    c.bench_function("Day 22, Part 2", |b| b.iter(|| day22::part2::solve(&input)));
}

criterion_group!(
    day22,
    benchmark_day22_input,
    benchmark_day22_part01,
    benchmark_day22_part02
);

//-------------------------------------------------------------------------------------
//Day 23-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day23_input(c: &mut Criterion) {
    c.bench_function("Day 23, Parse Input", |b| b.iter(day23::input::read));
}

pub fn benchmark_day23_part01(c: &mut Criterion) {
    let input = black_box(day23::input::read());
    c.bench_function("Day 23, Part 1", |b| b.iter(|| day23::part1::solve(&input)));
}

pub fn benchmark_day23_part02(c: &mut Criterion) {
    let input = black_box(day23::input::read());
    c.bench_function("Day 23, Part 2", |b| b.iter(|| day23::part2::solve(&input)));
}

criterion_group!(
    day23,
    benchmark_day23_input,
    benchmark_day23_part01,
    benchmark_day23_part02
);

//-------------------------------------------------------------------------------------
//Day 24-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day24_input(c: &mut Criterion) {
    c.bench_function("Day 24, Parse Input", |b| b.iter(day24::input::read));
}

pub fn benchmark_day24_part01(c: &mut Criterion) {
    let input = black_box(day24::input::read());
    c.bench_function("Day 24, Part 1", |b| b.iter(|| day24::part1::solve(&input)));
}

pub fn benchmark_day24_part02(c: &mut Criterion) {
    let input = black_box(day24::input::read());
    c.bench_function("Day 24, Part 2", |b| b.iter(|| day24::part2::solve(&input)));
}

criterion_group!(
    day24,
    benchmark_day24_input,
    benchmark_day24_part01,
    benchmark_day24_part02
);

//-------------------------------------------------------------------------------------
//Day 25-------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

pub fn benchmark_day25_input(c: &mut Criterion) {
    c.bench_function("Day 25, Parse Input", |b| b.iter(day25::input::read));
}

pub fn benchmark_day25_part01(c: &mut Criterion) {
    let input = black_box(day25::input::read());
    c.bench_function("Day 25, Part 1", |b| b.iter(|| day25::part1::solve(&input)));
}

criterion_group!(day25, benchmark_day25_input, benchmark_day25_part01);

//-------------------------------------------------------------------------------------
//Entrypoint---------------------------------------------------------------------------
//-------------------------------------------------------------------------------------

criterion_main!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
);
