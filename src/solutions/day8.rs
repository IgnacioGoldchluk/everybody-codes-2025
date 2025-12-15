use crate::solutions::solution;

pub struct Day8Solver;

impl solution::Solver for Day8Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let crosses = parse(input);
    let points = points();

    crosses
        .iter()
        .filter(|(from, to)| (from.abs_diff(*to)) == (points / 2))
        .count() as u64
}

fn part2(input: &str) -> u64 {
    let crosses = parse(input);

    crosses
        .iter()
        .enumerate()
        .map(|(idx, (x2, y2))| {
            crosses[..idx]
                .iter()
                .filter(|(x1, y1)| {
                    let (f1, t1) = (x1.min(y1), x1.max(y1));
                    let (f, t) = (x2.min(y2), x2.max(y2));
                    (f1 < f && f < t1 && t1 < t) || (f < f1 && f1 < t && t < t1)
                })
                .count() as u64
        })
        .sum()
}

fn part3(input: &str) -> u64 {
    let crosses = parse(input);

    let mut max = 0;

    for i in 1..points() {
        for j in i + 1..=points() {
            max = max.max(count_crosses(&(i, j), &crosses))
        }
    }

    max as u64
}

fn count_crosses(strike: &(u16, u16), crosses: &[(u16, u16)]) -> usize {
    let (f, t) = strike;
    crosses
        .iter()
        .filter(|(x1, y1)| {
            let (f1, t1) = (x1.min(y1), x1.max(y1));
            (f1 < f && f < t1 && t1 < t) || (f < f1 && f1 < t && t < t1) || (f == f1 && t == t1)
        })
        .count()
}

fn parse(input: &str) -> Vec<(u16, u16)> {
    let nums: Vec<u16> = input.split(",").map(|num| num.parse().unwrap()).collect();

    nums.iter()
        .zip(nums.iter().skip(1))
        .map(|(x, y)| (*x, *y))
        .collect()
}

fn points() -> u16 {
    if cfg!(test) { 8 } else { 256 }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_example() {
        let input_1 = "1,5,2,6,8,4,1,7,3";
        let input_2 = "1,5,2,6,8,4,1,7,3,5,7,8,2";
        let input_3 = "1,5,2,6,8,4,1,7,3";

        let input = solution::Input {
            part1: input_1.into(),
            part2: input_2.into(),
            part3: input_3.into(),
        };

        let solution = Day8Solver.solve(input);
        assert_eq!(solution.part1, "4");
        assert_eq!(solution.part2, "21");
        assert_eq!(solution.part3, "7");
    }
}
