use argh::FromArgs;
use solutions::solution::Solver;
use std::fs;

mod solutions;

#[derive(FromArgs)]
/// Executes the given Everybody Codes day
struct Args {
    /// the day to run
    #[argh(option)]
    day: u8,
}

fn main() {
    let args: Args = argh::from_env();

    let input = read_input(args.day);

    let solver: Box<dyn Solver> = match args.day {
        1 => Box::new(solutions::day1::Day1Solver {}),
        2 => Box::new(solutions::day2::Day2Solver {}),
        _ => todo!("Unreachable"),
    };

    let solution = solver.solve(input);
    println!("{}", solution);
}

fn read_input(day: u8) -> solutions::solution::Input {
    let file_path = format!("inputs/day{}", day);
    solutions::solution::Input {
        part1: fs::read_to_string(file_path.to_string() + "_1").unwrap(),
        part2: fs::read_to_string(file_path.to_string() + "_2").unwrap(),
        part3: fs::read_to_string(file_path.to_string() + "_3").unwrap(),
    }
}
