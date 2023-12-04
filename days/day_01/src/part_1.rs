fn process_line(line: &str) -> Option<u64> {
    let first_digit = line.chars().find_map(|ch| ch.to_digit(10))?;
    let last_digit = line.chars().rev().find_map(|ch| ch.to_digit(10))?;
    Some((first_digit * 10 + last_digit).into())
}

pub fn solution(input: &str) -> u64 {
    input.lines().map(process_line).flatten().sum()
}
