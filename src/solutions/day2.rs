use regex;
use std::str::FromStr;

use crate::solutions::solution;

#[derive(PartialEq, Eq)]
struct ComplexNumber {
    real: i64,
    imaginary: i64,
}

pub struct Day2Solver;

impl solution::Solver for Day2Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
            part3: part3(&input.part3),
        }
    }
}

fn part3(input: &str) -> String {
    let a = ComplexNumber::from_str(input).unwrap();
    let step = 1;

    grid(&a, step)
        .iter()
        .filter(|c| engrave(c))
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let a = ComplexNumber::from_str(input).unwrap();
    let step = 10;

    grid(&a, step)
        .iter()
        .filter(|c| engrave(c))
        .count()
        .to_string()
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

fn grid(number: &ComplexNumber, step: usize) -> Vec<ComplexNumber> {
    let mut v = Vec::new();

    for re in (number.real..=(number.real + 1000)).step_by(step) {
        for im in (number.imaginary..=(number.imaginary + 1000)).step_by(step) {
            v.push(ComplexNumber {
                real: re,
                imaginary: im,
            });
        }
    }
    v
}

fn engrave(number: &ComplexNumber) -> bool {
    let divisor = ComplexNumber {
        real: 100_000,
        imaginary: 100_000,
    };
    let mut start = ComplexNumber {
        real: 0,
        imaginary: 0,
    };
    for i in 1..=100 {
        if i > 2 && start == *number {
            return true;
        }

        start = start.multiply(&start).divide(&divisor).add(number);

        if diverges(&start) {
            return false;
        }
    }
    true
}

fn diverges(number: &ComplexNumber) -> bool {
    number.imaginary.abs() >= 1_000_000 || number.real.abs() >= 1_000_000
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
        let r = regex::Regex::new(r"\[(?P<real>-?\d+),(?P<imaginary>-?\d+)\]").unwrap();

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
        let input = solution::Input {
            part1: "A=[25,9]".into(),
            part2: "A=[35300,-64910]".into(),
            part3: "A=[35300,-64910]".into(),
        };

        let solution = Day2Solver.solve(input);
        assert_eq!(solution.part1, "[357,862]");
        assert_eq!(solution.part2, "4076");
        assert_eq!(solution.part3, "406954");
    }
}
