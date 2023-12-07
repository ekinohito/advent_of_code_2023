use crate::part_1;

pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines();
    let next_parsed_line = &mut || {
        lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
    };
    let time_line: Result<u64, _> = next_parsed_line();
    let time = time_line.expect("Error parsing time line");
    let distance_line: Result<u64, _> = next_parsed_line();
    let distance = distance_line.expect("Error parsing distance line");

    part_1::find_error_margin(time, distance)
}
