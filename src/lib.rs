#![allow(unused)]
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as DisplayResult};

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

macro_rules! impl_output_from {
    ( $( ($e:tt, $t:ty) ),* ) => {
        #[derive(Debug, Eq)]
        pub enum Output {
            $( $e($t), )*
        }

        $(
            impl From<$t> for Output {
                fn from(value: $t) -> Self {
                    Output::$e(value)
                }
            }
        )*
    };
}

impl_output_from!{
    (U8,     u8),
    (U16,    u16),
    (U32,    u32),
    (U64,    u64),
    (U128,   u128),
    (I8,     i8),
    (I16,    i16),
    (I32,    i32),
    (I64,    i64),
    (I128,   i128),
    (String, String)
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Output::U8(v) => write!(f, "{v}"),
            Output::U16(v) => write!(f, "{v}"),
            Output::U32(v) => write!(f, "{v}"),
            Output::U64(v) => write!(f, "{v}"),
            Output::U128(v) => write!(f, "{v}"),
            Output::I8(v) => write!(f, "{v}"),
            Output::I16(v) => write!(f, "{v}"),
            Output::I32(v) => write!(f, "{v}"),
            Output::I64(v) => write!(f, "{v}"),
            Output::I128(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}

/// Consider an output equal to any value where they can both be
/// coerced to the same string
impl<T: Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}
