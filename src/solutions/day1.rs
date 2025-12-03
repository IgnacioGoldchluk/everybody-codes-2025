use crate::solutions::solution;

pub struct Day1Solver;

enum Rotation {
    Left(i32),
    Right(i32),
}

fn part1(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let names: Vec<&str> = lines.next().unwrap().split(",").collect();
    let commands: Vec<Rotation> = lines.next().unwrap().split(",").map(to_rotation).collect();

    let size = names.len() as i32;

    let final_idx = commands.iter().fold(0, |pos, cmd| match cmd {
        Rotation::Left(v) => (pos - v).clamp(0, size - 1),
        Rotation::Right(v) => (pos + v).clamp(0, size - 1),
    });

    names[final_idx as usize].to_string()
}

fn part2(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let names: Vec<&str> = lines.next().unwrap().split(",").collect();
    let commands: Vec<Rotation> = lines.next().unwrap().split(",").map(to_rotation).collect();

    let size = names.len() as i32;

    let final_idx = commands.iter().fold(0, |pos, cmd| match cmd {
        Rotation::Left(v) => (pos - v).rem_euclid(size),
        Rotation::Right(v) => (pos + v).rem_euclid(size),
    });

    names[final_idx as usize].to_string()
}

fn part3(input: &str) -> String {
    let mut lines = input.split("\n\n");
    let names: Vec<&str> = lines.next().unwrap().split(",").collect();
    let commands: Vec<Rotation> = lines.next().unwrap().split(",").map(to_rotation).collect();

    let size = names.len() as i32;

    let mut indexes: Vec<usize> = (0..size as usize).collect();

    commands.iter().for_each(|cmd| match cmd {
        Rotation::Left(v) => indexes.swap(0, (size - v.rem_euclid(size)) as usize),
        Rotation::Right(v) => indexes.swap(0, v.rem_euclid(size) as usize),
    });

    names[indexes[0]].to_string()
}

impl solution::Solver for Day1Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2),
            part3: part3(&input.part3),
        }
    }
}

fn to_rotation(chars: &str) -> Rotation {
    let c = chars.chars().next().unwrap();
    let n: i32 = chars[1..].parse().unwrap();

    match c {
        'L' => Rotation::Left(n),
        'R' => Rotation::Right(n),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_solve() {
        let p1 = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#;

        let p2 = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#;

        let p3 = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L3"#;

        let input = solution::Input {
            part1: p1.into(),
            part2: p2.into(),
            part3: p3.into(),
        };

        let solution = Day1Solver.solve(input);
        assert_eq!(solution.part1, "Fyrryn");
        assert_eq!(solution.part2, "Elarzris");
        assert_eq!(solution.part3, "Drakzyph");
    }
}
