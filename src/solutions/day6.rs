use std::collections::HashMap;

use crate::solutions::solution;

pub struct Day6Solver;

impl solution::Solver for Day6Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: "".into(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut combinations = 0;
    let mut mentors = 0;

    for c in input.chars() {
        if c == 'A' {
            mentors += 1;
        } else if c == 'a' {
            combinations += mentors;
        }
    }
    combinations
}

fn part2(input: &str) -> u64 {
    let mut combinations = 0;
    let mut mentors: HashMap<char, u64> = HashMap::new();

    for c in input.chars() {
        if c.is_ascii_uppercase() {
            *mentors.entry(c).or_default() += 1;
        } else if c.is_ascii_lowercase() {
            combinations += *mentors.entry(c.to_ascii_uppercase()).or_default();
        } else {
            panic!("Unexpected character: '{c}'");
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let input = solution::Input {
            part1: "ABabACacBCbca".into(),
            part2: "ABabACacBCbca".into(),
            part3: "".into(),
        };

        let solution = Day6Solver.solve(input);
        assert_eq!(solution.part1, "5");
        assert_eq!(solution.part2, "11");
        assert_eq!(solution.part3, "");
    }
}
