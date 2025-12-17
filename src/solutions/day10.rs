use crate::solutions::solution;
use std::collections::HashSet;

pub struct Day10Solver;

type Point = (usize, usize);

struct Grid {
    sheep: Vec<u128>,
    safe: Vec<u128>,
    dragon: Point,
    limits: Point,
}

impl solution::Solver for Day10Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: "".into(),
        }
    }
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from(input);

    let moves = moves(2);
    let mut move_n = 0;
    let mut to_visit = HashSet::from([grid.dragon]);
    let mut sheep_pos: HashSet<Point> = HashSet::new();

    while move_n != (moves + 1) {
        let mut new_moves = HashSet::new();
        for pos in to_visit.iter() {
            if grid.has_safe_space(pos) || move_n as usize > pos.0 {
                continue;
            }
            sheep_pos.extend(
                [
                    (pos.0 - move_n as usize + 1, pos.1),
                    (pos.0 - move_n as usize, pos.1),
                ]
                .iter()
                .filter(|pos| grid.has_sheep(pos)),
            );
            new_moves.extend(grid.next_moves(pos));
        }

        to_visit = new_moves;
        move_n += 1;
    }

    sheep_pos.len() as u64
}

fn part1(input: &str) -> u64 {
    let grid = Grid::from(input);

    let moves = moves(1);
    let mut move_n = 0;
    let mut to_visit = vec![grid.dragon];
    let mut sheep_pos: HashSet<Point> = HashSet::new();

    while move_n != (moves + 1) {
        let mut new_moves = vec![];
        for pos in to_visit.iter() {
            if grid.can_eat_sheep(pos) {
                sheep_pos.insert(*pos);
            }
            new_moves.extend(grid.next_moves(pos));
        }

        to_visit = new_moves;
        move_n += 1;
    }

    sheep_pos.len() as u64
}

fn moves(part: u8) -> u8 {
    match (part, cfg!(test)) {
        (1, true) => 3,
        (1, false) => 4,
        (2, true) => 3,
        (2, false) => 20,
        _ => unreachable!(),
    }
}

impl Grid {
    pub fn has_sheep(&self, (x, y): &Point) -> bool {
        (self.sheep[*x] & (1 << y)) != 0
    }

    pub fn has_safe_space(&self, (x, y): &Point) -> bool {
        (self.safe[*x] & (1 << y)) != 0
    }

    pub fn can_eat_sheep(&self, pos: &Point) -> bool {
        self.has_sheep(pos) && !self.has_safe_space(pos)
    }

    pub fn next_moves(&self, dragon_posiiton: &Point) -> Vec<Point> {
        let x = dragon_posiiton.0 as i32;
        let y = dragon_posiiton.1 as i32;
        let (max_x, max_y) = self.limits;
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
            .filter(|(dx, dy)| x + dx >= 0 && y + dy >= 0)
            .map(|(dx, dy)| ((x + *dx) as usize, (y + *dy) as usize))
            .filter(|(new_x, new_y)| *new_x <= max_x && *new_y <= max_y)
            .collect()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut dragon = (0, 0);
        let mut limits = (0, 0);
        let mut safe = vec![];
        let mut sheep = vec![];
        for (row, line) in value.lines().enumerate() {
            let mut sheep_row = 0;
            let mut safe_row = 0;
            for (col, c) in line.char_indices() {
                match c {
                    'D' => dragon = (row, col),
                    '#' => safe_row |= 1 << col,
                    'S' => sheep_row |= 1 << col,
                    '.' => (),
                    _ => unreachable!(),
                }
                limits = (row, col);
            }
            sheep.push(sheep_row);
            safe.push(safe_row);
        }

        Self {
            dragon,
            safe,
            sheep,
            limits,
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

        let input_2 = r####"...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S.."####;

        let input = solution::Input {
            part1: input_1.into(),
            part2: input_2.into(),
            part3: "".into(),
        };

        let solution = Day10Solver.solve(input);
        assert_eq!(solution.part1, "27");
        assert_eq!(solution.part2, "27");
    }
}
