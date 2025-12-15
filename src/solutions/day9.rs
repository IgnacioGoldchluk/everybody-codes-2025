use crate::solutions::solution;

pub struct Day9Solver;

impl solution::Solver for Day9Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: "".into(),
            part3: "".into(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let (child, parents) = parse(input);

    parents
        .iter()
        .map(|p| p.iter().zip(&child).filter(|(c1, c2)| *c1 == *c2).count() as u64)
        .product()
}

fn parse(input: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let mut dnas = input
        .lines()
        .rev()
        .map(|l| l.split(":").nth(1).unwrap().chars().collect::<Vec<char>>());
    let child = dnas.next().unwrap();

    (child, dnas.collect())
}

#[cfg(test)]
mod tests {

    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_input() {
        let input_1 = r#"1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG"#;

        let input = solution::Input {
            part1: input_1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day9Solver.solve(input);
        assert_eq!(solution.part1, "414");
    }
}
