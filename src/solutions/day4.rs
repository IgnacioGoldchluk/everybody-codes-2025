use crate::solutions::solution;

pub struct Day4Solver;

impl solution::Solver for Day4Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
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

fn part3(input: &str) -> u64 {
    let gears = parse_3(input);
    let final_ratio: u64 = gears[1..gears.len() - 1].iter().product();

    (100 * gears[0] * final_ratio) / gears[gears.len() - 1]
}

fn parse_3(input: &str) -> Vec<u64> {
    input.lines().map(to_gear_ratio).collect()
}

fn to_gear_ratio(input: &str) -> u64 {
    let nums = input.split("|").collect::<Vec<&str>>();

    match nums[..] {
        [x] => x.parse().unwrap(),
        [l, r] => {
            let right: u64 = r.parse().unwrap();
            let left: u64 = l.parse().unwrap();
            right / left
        }
        _ => panic!("Unknown result {input}"),
    }
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

        let p3 = r#"5
7|21
18|36
27|27
10|50
10|50
11"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p1.into(),
            part3: p3.into(),
        };

        let solution = Day4Solver.solve(input);

        assert_eq!(solution.part1, "15888");
        assert_eq!(solution.part2, "1274509803922");
        assert_eq!(solution.part3, "6818");
    }
}
