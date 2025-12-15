use crate::solutions::solution;
use disjoint::DisjointSet;

pub struct Day9Solver;

impl solution::Solver for Day9Solver {
    fn solve(&self, input: solution::Input) -> solution::Solution {
        solution::Solution {
            part1: part1(&input.part1).to_string(),
            part2: part2(&input.part2).to_string(),
            part3: part3(&input.part3).to_string(),
        }
    }
}

type Dna = Vec<char>;

fn part1(input: &str) -> u64 {
    let (child, parents) = parse1(input);

    parents
        .iter()
        .map(|p| p.iter().zip(&child).filter(|(c1, c2)| *c1 == *c2).count() as u64)
        .product()
}

fn part2(input: &str) -> u64 {
    let dnas = parse(input);
    let mut total = 0;

    for (ic, child) in dnas.iter().enumerate() {
        for (ip1, p1) in dnas.iter().enumerate() {
            for (ip2, p2) in dnas.iter().enumerate() {
                if ic == ip1 || ic == ip2 || ip1 <= ip2 {
                    continue;
                }
                if is_child(child, p1, p2) {
                    total += similarity(child, p1, p2);
                }
            }
        }
    }

    total
}

fn part3(input: &str) -> u64 {
    let dnas = parse(input);
    let mut families = DisjointSet::with_len(dnas.len());

    for (ic, child) in dnas.iter().enumerate() {
        for (ip1, p1) in dnas.iter().enumerate() {
            for (ip2, p2) in dnas.iter().enumerate() {
                if ic == ip1 || ic == ip2 || ip1 <= ip2 {
                    continue;
                }
                if is_child(child, p1, p2) {
                    families.join(ic, ip1);
                    families.join(ic, ip2);
                }
            }
        }
    }

    families
        .sets()
        .iter()
        .max_by(|f1, f2| f1.len().cmp(&f2.len()))
        .map(|values| values.iter().map(|x| x + 1).sum::<usize>())
        .unwrap() as u64
}

fn is_child(child: &Dna, p1: &Dna, p2: &Dna) -> bool {
    child
        .iter()
        .enumerate()
        .all(|(idx, c)| *c == p1[idx] || *c == p2[idx])
}

fn similarity(child: &Dna, p1: &Dna, p2: &Dna) -> u64 {
    [p1, p2]
        .iter()
        .map(|p| p.iter().zip(child).filter(|(c1, c2)| *c1 == *c2).count() as u64)
        .product()
}

fn parse(input: &str) -> Vec<Dna> {
    input
        .lines()
        .map(|l| l.split(":").nth(1).unwrap().chars().collect())
        .collect()
}

fn parse1(input: &str) -> (Dna, Vec<Dna>) {
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

        let input_2 = r#"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"#;

        let input_3 = r#"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG"#;

        let input = solution::Input {
            part1: input_1.into(),
            part2: input_2.into(),
            part3: input_3.into(),
        };

        let solution = Day9Solver.solve(input);
        assert_eq!(solution.part1, "414");
        assert_eq!(solution.part2, "1245");
        assert_eq!(solution.part3, "36")
    }
}
