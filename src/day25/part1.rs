use super::input::Snafu;
use crate::day25::{Input, Output};

/// Solve Day 25, Part 1
pub fn solve(input: &Input) -> Output {
    let fuel_cost: i128 = input.iter().cloned().map(i128::from).sum();
    let snafu_cost = Snafu::from(fuel_cost);
    snafu_cost.0.into()
}

/// Unit conversion from a SNAFU number to a base-10 integer. This is a pretty
/// common algorithm for converting base-whatever to decimal.
impl From<Snafu> for i128 {
    fn from(snafu: Snafu) -> Self {
        let mut total = 0;
        for (pow, glyph) in snafu.0.chars().rev().enumerate() {
            let mult = match glyph {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            };
            total += 5i128.pow(pow as u32) * mult;
        }
        total
    }
}

/// Unit conversion from a decimal integer into a SNAFU number. This is a much
/// less common (to me) operation. The tricky bit is handling the fact that we have
/// digits whose value is centered around zero instead of anchored at zero on the
/// least significant place. For a normal base-5 conversion, I'd take the result
/// of the remainder % 5, then divide by 5 for each digit until no remainder was
/// left. Apparently, adding two each round lets us shift the digits. I'm not
/// 100% sure why this works, but it works. Math, amirite? This solution was inspired
/// by Google searches about "balanced ternary" number systems.
impl From<i128> for Snafu {
    fn from(number: i128) -> Self {
        let mut remainder = number;
        let mut out = String::default();
        while remainder > 0 {
            let glyph = match remainder % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            };
            out.push(glyph);
            remainder += 2; // This works for some reason.
            remainder /= 5;
        }
        Snafu(out.chars().rev().collect::<String>())
    }
}
