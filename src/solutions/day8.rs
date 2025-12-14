use crate::solutions::solution;

pub struct Day8Solver;

impl solution::Solver for Day8Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: "".into(),
            part3: "".into(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let crosses = parse(input);
    let points = 32;

    crosses
        .iter()
        .zip(crosses.iter().skip(1))
        .filter(|(from, to)| (from.abs_diff(**to)) == (points / 2))
        .count() as u64
}

fn parse(input: &str) -> Vec<u8> {
    input.split(",").map(|num| num.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_example() {
        let input_1 = "1,5,2,6,8,4,1,7,3";

        let input = solution::Input {
            part1: input_1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day8Solver.solve(input);
    }
}
