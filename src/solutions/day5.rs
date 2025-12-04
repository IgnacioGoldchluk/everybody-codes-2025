use crate::solutions::solution;
use std::{cmp::Ordering, fmt::Error, str::FromStr};

pub struct Day5Solver;

#[derive(PartialEq, Eq)]
struct Segment(Option<u64>, u64, Option<u64>);

struct Sword {
    identifier: u64,
    fishbone: Vec<Segment>,
}

impl solution::Solver for Day5Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
        }
    }
}

fn part3(input: &str) -> u64 {
    let mut swords: Vec<Sword> = input.lines().map(|s| Sword::from_str(s).unwrap()).collect();
    swords.sort();
    swords.reverse();

    swords
        .iter()
        .enumerate()
        .map(|(idx, sword)| sword.identifier * (idx + 1) as u64)
        .sum()
}

fn part2(input: &str) -> u64 {
    let swords: Vec<Sword> = input.lines().map(|s| Sword::from_str(s).unwrap()).collect();

    let mut qualities: Vec<u64> = swords.iter().map(|s| s.quality()).collect();
    qualities.sort();

    qualities[qualities.len() - 1] - qualities[0]
}

fn part1(input: &str) -> u64 {
    Sword::from_str(input.lines().next().unwrap())
        .unwrap()
        .quality()
}

fn fishbone(numbers: &[u64]) -> Vec<Segment> {
    let mut fishbone = Vec::new();

    for n in numbers {
        add_to_fishbone(&mut fishbone, *n);
    }

    fishbone
}

fn add_to_fishbone(fishbone: &mut Vec<Segment>, number: u64) {
    let mut im = fishbone.iter_mut();

    loop {
        let s = im.next();

        match s {
            None => {
                fishbone.push(Segment(None, number, None));
                break;
            }
            Some(Segment(None, m, _)) if number < *m => {
                s.unwrap().0 = Some(number);
                break;
            }
            Some(Segment(_, m, None)) if number > *m => {
                s.unwrap().2 = Some(number);
                break;
            }
            _ => (),
        }
    }
}

// This should be moved to a Segment -> u64 trait
fn segment_to_number(segment: &Segment) -> u64 {
    match &segment {
        Segment(None, n, None) => *n,
        Segment(Some(l), n, None) => format!("{}{}", l, n).parse().unwrap(),
        Segment(None, n, Some(r)) => format!("{}{}", n, r).parse().unwrap(),
        Segment(Some(l), n, Some(r)) => format!("{}{}{}", l, n, r).parse().unwrap(),
    }
}

impl Sword {
    fn quality(&self) -> u64 {
        self.fishbone
            .iter()
            .map(|segment| segment.1.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap()
    }
}

impl FromStr for Sword {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = s.split(":");
        let identifier = parser.next().unwrap().parse().unwrap();

        let numbers: Vec<u64> = parser
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            identifier,
            fishbone: fishbone(&numbers),
        })
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.quality().cmp(&other.quality()) {
            Ordering::Equal => {
                for (f1, f2) in self.fishbone.iter().zip(&other.fishbone) {
                    let first = segment_to_number(f1);
                    let second: u64 = segment_to_number(f2);

                    match first.cmp(&second) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }

                self.identifier.cmp(&other.identifier)
            }
            other => other,
        }
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sword {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && (self.fishbone == other.fishbone)
    }
}

impl Eq for Sword {}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_input() {
        let p2 = r#"1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5"#;

        let p3 = r#"1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7"#;

        let input = solution::Input {
            part1: "58:5,3,7,8,9,10,4,5,7,8,8".into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day5Solver.solve(input);

        assert_eq!(solution.part1, "581078");
        assert_eq!(solution.part2, "77053");
        assert_eq!(solution.part3, "260");
    }
}
