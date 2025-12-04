use crate::solutions::solution;
use std::collections::{HashMap, HashSet};

pub struct Day3Solver;

impl solution::Solver for Day3Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&parse(&input.part1)).to_string(),
            part2: part2(&parse(&input.part2)).to_string(),
            part3: part3(&parse(&input.part3)).to_string(),
        }
    }
}

fn part1(nums: &[u32]) -> u32 {
    let x: HashSet<u32> = HashSet::from_iter(nums.iter().cloned());

    x.iter().sum()
}

fn part2(nums: &[u32]) -> u32 {
    let mut n: Vec<u32> = nums.to_vec();

    n.sort();
    n.dedup();

    n.iter().take(20).sum()
}

fn part3(nums: &[u32]) -> u32 {
    let mut freqs = HashMap::new();

    for n in nums {
        *freqs.entry(n).or_default() += 1;
    }

    *freqs.values().max().unwrap()
}

fn parse(input: &str) -> Vec<u32> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_input() {
        let input = solution::Input {
            part1: "10,5,1,10,3,8,5,2,2".into(),
            part2: "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77".into(),
            part3: "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77".into(),
        };

        let solution = Day3Solver.solve(input);

        assert_eq!(solution.part1, "29");
        assert_eq!(solution.part2, "781");
        assert_eq!(solution.part3, "3");
    }
}
