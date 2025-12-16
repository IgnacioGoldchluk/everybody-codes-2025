use crate::solutions::solution;
use std::collections::{HashMap, HashSet};

pub struct Day10Solver;

enum Element {
    Dragon,
    Sheep,
    Empty,
    Safe,
}

type Point = (i32, i32);

type Grid = HashMap<Point, Element>;

impl solution::Solver for Day10Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: "".into(),
            part3: "".into(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let grid = parse(input);

    let unique_positions: HashSet<Point> = valid_positions(&find_dragon(&grid), moves())
        .iter()
        .map(|(_move_num, pos)| *pos)
        .collect();

    unique_positions
        .iter()
        .filter(|pos| matches!(grid.get(pos), Some(Element::Sheep)))
        .count() as u64
}

fn valid_positions(start: &Point, total_moves: u8) -> HashSet<(u8, Point)> {
    let mut moves_left = total_moves;
    let mut to_visit = vec![(0, *start)];
    let mut positions = HashSet::new();

    while moves_left != 0 {
        let new_moves: Vec<(u8, Point)> = to_visit
            .iter()
            .flat_map(|(move_n, pos)| {
                next_moves(pos)
                    .iter()
                    .map(|m| (move_n + 1, *m))
                    .collect::<Vec<(u8, Point)>>()
            })
            .collect();
        positions.extend(&new_moves);
        to_visit = new_moves;

        moves_left -= 1;
    }

    positions
}

fn find_dragon(grid: &Grid) -> Point {
    grid.iter()
        .find_map(|(coord, el)| match el {
            Element::Dragon => Some(*coord),
            _ => None,
        })
        .unwrap()
}

fn next_moves(pos: &Point) -> Vec<Point> {
    let (curr_x, curr_y) = pos;
    let mov = [
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
        (-2, 1),
        (-2, -1),
    ];

    mov.iter()
        .map(|(x, y)| (curr_x + x, curr_y + y))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .collect()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, c)| ((row as i32, col as i32), Element::from(c)))
        })
        .collect()
}

fn moves() -> u8 {
    if cfg!(test) { 3 } else { 4 }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            'D' => Self::Dragon,
            'S' => Self::Sheep,
            '.' => Self::Empty,
            '#' => Self::Safe,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::solution::Solver;
    use super::*;

    #[test]
    fn test_example() {
        let input_1 = r#"...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S.."#;

        let input = solution::Input {
            part1: input_1.into(),
            part2: "".into(),
            part3: "".into(),
        };

        let solution = Day10Solver.solve(input);
        assert_eq!(solution.part1, "27");
    }
}
