use std::collections::HashMap;

use crate::solutions::solution;

pub struct Day6Solver;

type Frequencies = HashMap<char, u64>;

impl solution::Solver for Day6Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
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
    let mut mentors: Frequencies = HashMap::new();

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

fn part3(input: &str) -> u64 {
    let characters: Vec<char> = input.chars().collect();
    let length = characters.len();
    let window_size = 1000;
    let mut combinations = 0;

    for (i, c) in characters.iter().enumerate() {
        if !c.is_ascii_lowercase() {
            continue;
        }
        let mentor = c.to_ascii_uppercase();

        for n in -window_size..=window_size {
            // If it wrapped then add 999, else 1000
            let idx_unrwapped = (i as i64) + n;
            let idx = idx_unrwapped.rem_euclid(length as i64);

            if characters[idx as usize] == mentor {
                let mult = if idx == idx_unrwapped { 1000 } else { 999 };
                combinations += mult;
            }
        }
    }

    combinations as u64
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
            part3: "AABCBABCABCabcabcABCCBAACBCa".into(),
        };

        let solution = Day6Solver.solve(input);
        assert_eq!(solution.part1, "5");
        assert_eq!(solution.part2, "11");
    }
}
