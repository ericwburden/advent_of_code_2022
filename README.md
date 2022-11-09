# Eric's Advent of Code 2022 Solutions

## The Blog

For the last two years, I've blogged my approaches to the Advent of Code puzzles on my
[personal site](https://www.ericburden.work/blog/). Assuming I hold true to form, each 
blog post will include code and commentary on my thinking behind the approach, my thoughts
about the puzzles, and vain attempts at wit.

## Project Structure

This year, I'm using Rust! I solved 2019's puzzles in Rust after the fact (it's how I
learned Rust to begin with), but this year I'll solve each day in Rust first. I've 
set up folders for each day's code and input files like so:

```
<project root>
├─input
│ └─XX
│   ├─input.txt
│   └─test.txt
├─src
│ └─dayXX
│   ├─input.rs
│   ├─mod.rs
│   ├─part1.rs
│   └─part2.rs
├─Cargo.toml
└─README.md
```

At present, the `main` method doesn't do anything, though I have plans to make it into a
CLI to run the code for particular days, similar to my setup for 2021.

There are a few organizational notes to point out here:

- The `mod.rs` file for each day defines `Input` as a type alias for the type the
  input file will be parsed into, `Output` as a type alias for the puzzle answer type
  (usually a `u32` or `i32`), and a convenience function `run(_: Part) -> Output`
  that reads in the input and solves for either part one or part two, depending on the
  variant of `Part` that is passed. This file also contains the tests that cofirm the
  answer once it has been found.
- Input files are being included in each day's `input.rs` via the `include_str!()` macro,
  which means parsing will be on the file contents as one long, newline-separated, string
  slice. The main entrypoint for input parsing is the `read() -> Input` function which
  takes no arguments (relying on the included `INPUT` constant) and returns the parsed
  input file.
- The `part1.rs` and `part2.rs` files each contain a `solve(_: &Input) -> Output` function
  that takes a reference to the parsed input and returns the solution for that part of
  that day.

 
