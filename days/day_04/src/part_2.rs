use std::collections::HashSet;

fn solution_line(line: &str) -> usize {
    let (_id, rest) = line.split_once(':').unwrap();
    // let (_, id) = id.split_once(':').unwrap();
    let (winning, chosen) = rest.split_once('|').unwrap();
    fn parse_set(from: &str) -> HashSet<u64> {
        from.trim().split_whitespace().map(|num| num.parse().unwrap()).collect()
    }
    let winning = parse_set(winning);
    let chosen = parse_set(chosen);
    winning.intersection(&chosen).count()
}

pub fn solution(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut copies = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let solution = solution_line(line);
        let addition = copies[i];
        for copy in &mut copies[i + 1..=i + solution] {
            *copy += addition;
        }
    }
    copies.iter().sum()
}