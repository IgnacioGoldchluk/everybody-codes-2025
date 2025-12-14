use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::solutions::solution;

pub struct Day7Solver;

impl solution::Solver for Day7Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2).to_string(),
            part3: "".into(),
        }
    }
}

type Rules = HashMap<char, HashSet<char>>;

fn part1(input: &str) -> String {
    let mut i = input.split("\n\n");
    let names: Vec<&str> = i.next().unwrap().split(",").collect();
    let rules = parse_rules(i.next().unwrap());

    names
        .iter()
        .find(|name| matches_rules(name, &rules))
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> u64 {
    let mut i = input.split("\n\n");
    let names: Vec<&str> = i.next().unwrap().split(",").collect();
    let rules = parse_rules(i.next().unwrap());

    names
        .iter()
        .enumerate()
        .filter(|(_i, name)| matches_rules(name, &rules))
        .map(|(idx, _name)| (idx + 1) as u64)
        .sum()
}

fn matches_rules(name: &str, rules: &Rules) -> bool {
    for (l, r) in name.chars().zip(name.chars().skip(1)) {
        if !rules.get(&l).unwrap().contains(&r) {
            return false;
        }
    }

    true
}

fn parse_rules(rules: &str) -> Rules {
    let regex = Regex::new(r"(?P<lead>\w)\s>\s(?P<followers>[\w,]+)").unwrap();

    rules
        .split("\n")
        .map(|line| {
            let capts = regex.captures(line).unwrap();
            let lead = capts["lead"].chars().next().unwrap();
            let followers = capts["followers"]
                .split(",")
                .map(|c| c.chars().next().unwrap());

            (lead, HashSet::from_iter(followers))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solution::Solver;
    use super::*;
    #[test]
    fn test_input() {
        let input_1 = r#"Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h"#;

        let input_2 = r#"Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"#;

        let input = solution::Input {
            part1: input_1.into(),
            part2: input_2.into(),
            part3: "".into(),
        };

        let solution = Day7Solver.solve(input);
        assert_eq!(solution.part1, "Oroneth");
    }
}
