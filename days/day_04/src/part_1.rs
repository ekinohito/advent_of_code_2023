use std::collections::HashSet;

fn solution_line(line: &str) -> u64 {
    let (_id, rest) = line.split_once(':').unwrap();
    // let (_, id) = id.split_once(':').unwrap();
    let (winning, chosen) = rest.split_once('|').unwrap();
    fn parse_set(from: &str) -> HashSet<u64> {
        from.trim().split_whitespace().map(|num| num.parse().unwrap()).collect()
    }
    let winning = parse_set(winning);
    let chosen = parse_set(chosen);
    match winning.intersection(&chosen).count() {
        0 => 0,
        rest => 1 << rest
    }
}

pub fn solution(input: &str) -> u64 {
    input.lines().map(solution_line).sum()
}