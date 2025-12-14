use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::solutions::solution;

pub struct Day7Solver;

impl solution::Solver for Day7Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
        }
    }
}

type Rules = HashMap<char, HashSet<char>>;

fn part1(input: &str) -> String {
    let (names, rules) = parse(input);

    names
        .iter()
        .find(|name| matches_rules(name, &rules))
        .unwrap()
        .to_string()
}

fn part2(input: &str) -> u64 {
    let (names, rules) = parse(input);

    names
        .iter()
        .enumerate()
        .filter(|(_i, name)| matches_rules(name, &rules))
        .map(|(idx, _name)| (idx + 1) as u64)
        .sum()
}

fn part3(input: &str) -> u64 {
    let (prefixes, rules) = parse(input);

    remove_substrings(&prefixes)
        .iter()
        .filter(|name| matches_rules(name, &rules))
        .map(|name| combinations(name, &rules))
        .sum()
}

fn combinations(name: &str, rules: &Rules) -> u64 {
    // If 7 <= length <= 10 -> 1 + sum (because already valid)
    // If
    let length = name.len();
    if length == 11 {
        return 1;
    }

    let last_char = name.chars().last().unwrap();
    let default = HashSet::new();
    let nexts = rules.get(&last_char).unwrap_or(&default);
    let to_add = if (7..=10).contains(&length) { 1 } else { 0 };

    let sum: u64 = nexts
        .iter()
        .map(|c| combinations(&(name.to_string() + &c.to_string()), rules))
        .sum();

    sum + to_add
}

fn remove_substrings(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .filter(|candidate| {
            !names
                .iter()
                .any(|other| candidate.starts_with(other) && candidate.to_string() != *other)
        })
        .map(|s| s.to_string())
        .collect()
}

fn matches_rules(name: &str, rules: &Rules) -> bool {
    for (l, r) in name.chars().zip(name.chars().skip(1)) {
        if !rules.get(&l).unwrap().contains(&r) {
            return false;
        }
    }

    true
}

fn parse(input: &str) -> (Vec<&str>, Rules) {
    let mut i = input.split("\n\n");
    let names: Vec<&str> = i.next().unwrap().split(",").collect();
    let rules = parse_rules(i.next().unwrap());

    (names, rules)
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

        let input_3 = r#"Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
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
            part3: input_3.into(),
        };

        let solution = Day7Solver.solve(input);
        assert_eq!(solution.part1, "Oroneth");
        assert_eq!(solution.part2, "23");
        assert_eq!(solution.part3, "1154");
    }
}
