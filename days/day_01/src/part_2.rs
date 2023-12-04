const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line(line: &str) -> Option<u64> {
    let first_digit = line.char_indices().find_map(|(i, ch)| {
        ch.to_digit(10).or_else(|| {
            let slice = &line[i..];
            DIGITS
                .iter()
                .position(|digit| slice.starts_with(*digit))
                .map(|digit| digit as u32 + 1)
        })
    })?;
    let last_digit = line.char_indices().rev().find_map(|(i, ch)| {
        ch.to_digit(10).or_else(|| {
            let slice = &line[..=i];
            DIGITS
                .iter()
                .position(|digit| slice.ends_with(*digit))
                .map(|digit| digit as u32 + 1)
        })
    })?;
    Some((first_digit * 10 + last_digit).into())
}

#[test]
fn line() {
    assert_eq!(process_line("fivefdlqonesj2six"), Some(56))
}

#[test]
fn more_lines() {
    assert_eq!(solution(r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#), 281);
}

pub fn solution(input: &str) -> u64 {
    input.lines().map(process_line).map(|x| x.unwrap()).sum()
}
