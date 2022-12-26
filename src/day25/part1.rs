use super::input::Snafu;
use crate::day25::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let fuel_cost: i128 = input.iter().cloned().map(i128::from).sum();
    let snafu_cost = Snafu::from(fuel_cost);
    snafu_cost.0.into()
}

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
            remainder += 2;
            remainder /= 5;
        }
        Snafu(out.chars().rev().collect::<String>())
    }
}
// def g(n):
//     return "" if n == 0 else g((n + 2) // 5) + "012=-"[n % 5]
