use crate::solutions::solution;

pub struct Day4Solver;

impl solution::Solver for Day4Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: "0".to_string(),
        }
    }
}

fn part1(input: &str) -> u32 {
    let nums = parse(input);
    2025 * nums[0] / nums[nums.len() - 1]
}

fn part2(input: &str) -> f64 {
    let nums = parse(input);

    ((10_000_000_000_000f64 / (nums[0] as f64)) * nums[nums.len() - 1] as f64).ceil()
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_input() {
        let p1 = r#"102
75
50
35
13"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77".into(),
        };

        let solution = Day4Solver.solve(input);

        assert_eq!(solution.part1, "15888");
        assert_eq!(solution.part2, "1274509803922");
    }
}
