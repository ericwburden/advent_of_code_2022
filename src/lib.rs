#![allow(unused)]
use std::fmt::{Display, Formatter, Result as DisplayResult};
use std::cmp::PartialEq;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

// Used as a flag to indicate which part of a day to run.
pub enum Part {
    One,
    Two,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Output {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    String(String),
}

impl From<u32> for Output {
    fn from(value: u32) -> Self {
        Output::U32(value)
    }
}

impl From<i32> for Output {
    fn from(value: i32) -> Self {
        Output::I32(value)
    }
}

impl From<u64> for Output {
    fn from(value: u64) -> Self {
        Output::U64(value)
    }
}

impl From<i64> for Output {
    fn from(value: i64) -> Self {
        Output::I64(value)
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        Output::String(value)
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Output::U32(v) => write!(f, "{v}"),
            Output::I32(v) => write!(f, "{v}"),
            Output::U64(v) => write!(f, "{v}"),
            Output::I64(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}

impl PartialEq<u32> for Output {
    fn eq(&self, other: &u32) -> bool {
        let Output::U32(value) = self else { return false; };
        value == other
    }
}

impl PartialEq<Output> for u32 {
    fn eq(&self, other: &Output) -> bool {
        let Output::U32(value) = other else { return false; };
        value == self
    }
}

impl PartialEq<i32> for Output {
    fn eq(&self, other: &i32) -> bool {
        let Output::I32(value) = self else { return false; };
        value == other
    }
}

impl PartialEq<Output> for i32 {
    fn eq(&self, other: &Output) -> bool {
        let Output::I32(value) = other else { return false; };
        value == self
    }
}

impl PartialEq<u64> for Output {
    fn eq(&self, other: &u64) -> bool {
        let Output::U64(value) = self else { return false; };
        value == other
    }
}

impl PartialEq<Output> for u64 {
    fn eq(&self, other: &Output) -> bool {
        let Output::U64(value) = other else { return false; };
        value == self
    }
}

impl PartialEq<i64> for Output {
    fn eq(&self, other: &i64) -> bool {
        let Output::I64(value) = self else { return false; };
        value == other
    }
}

impl PartialEq<Output> for i64 {
    fn eq(&self, other: &Output) -> bool {
        let Output::I64(value) = other else { return false; };
        value == self
    }
}

impl PartialEq<&str> for Output {
    fn eq(&self, other: &&str) -> bool {
        let Output::String(value) = self else { return false; };
        value == other
    }
}

impl PartialEq<Output> for &str {
    fn eq(&self, other: &Output) -> bool {
        let Output::String(value) = other else { return false; };
        value == self
    }
}
