use regex;
use std::str::FromStr;

use crate::solutions::solution;

struct ComplexNumber {
    real: i64,
    imaginary: i64,
}

pub struct Day2Solver;

impl solution::Solver for Day2Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: "0".into(),
            part3: "0".into(),
        }
    }
}

fn part1(input: &str) -> String {
    let a = ComplexNumber::from_str(input).unwrap();

    let tenten = ComplexNumber {
        real: 10,
        imaginary: 10,
    };

    let result = (1..=3).fold(
        ComplexNumber {
            real: 0,
            imaginary: 0,
        },
        |acc, _| acc.multiply(&acc).divide(&tenten).add(&a),
    );

    format!("[{},{}]", result.real, result.imaginary)
}

impl ComplexNumber {
    fn add(&self, other: &Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        // [X1,Y1] * [X2,Y2] = [X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2]
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    fn divide(&self, other: &Self) -> Self {
        // [X1,Y1] / [X2,Y2] = [X1 / X2, Y1 / Y2]
        Self {
            real: self.real / other.real,
            imaginary: self.imaginary / other.imaginary,
        }
    }
}

#[derive(Debug)]
struct ParseError;

impl FromStr for ComplexNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"\[(?P<real>\d+),(?P<imaginary>\d+)\]").unwrap();

        let caps = r.captures(s).unwrap();
        Ok(Self {
            real: caps["real"].parse().unwrap(),
            imaginary: caps["imaginary"].parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = "A=[25,9]";

        let input = solution::Input {
            part1: p1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day2Solver.solve(input);
        assert_eq!(solution.part1, "[357,862]");
        assert_eq!(solution.part2, "0");
        assert_eq!(solution.part3, "0");
    }
}
