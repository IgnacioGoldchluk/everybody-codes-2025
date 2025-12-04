use crate::solutions::solution;

pub struct Day5Solver;

struct Segment(Option<u64>, u64, Option<u64>);

impl solution::Solver for Day5Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: "".to_string(),
        }
    }
}

fn part2(input: &str) -> u64 {
    let mut qualities: Vec<u64> = input.lines().map(quality).collect();
    qualities.sort();

    qualities[qualities.len() - 1] - qualities[0]
}

fn part1(input: &str) -> u64 {
    quality(input.lines().next().unwrap())
}

fn quality(sword: &str) -> u64 {
    let (_identifier, numbers) = parse(sword);

    let mut fishbone = Vec::new();

    for n in numbers {
        add_to_fishbone(&mut fishbone, n);
    }

    fishbone
        .iter()
        .map(|segment| segment.1.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap()
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

fn parse(input: &str) -> (u64, Vec<u64>) {
    let mut parser = input.split(":");
    let identifier = parser.next().unwrap().parse().unwrap();

    let numbers = parser
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    (identifier, numbers)
}

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

        let input = solution::Input {
            part1: "58:5,3,7,8,9,10,4,5,7,8,8".into(),
            part2: p2.into(),
            part3: "".into(),
        };

        let solution = Day5Solver.solve(input);

        assert_eq!(solution.part1, "581078");
        assert_eq!(solution.part2, "77053");
        assert_eq!(solution.part3, "");
    }
}
