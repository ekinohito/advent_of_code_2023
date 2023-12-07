pub fn find_error_margin(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;
    let half_time = time / 2.;
    let discriminant = (half_time * half_time) - distance;
    assert!(discriminant > 0.);
    let margin = discriminant.sqrt();
    let start = half_time - margin;
    let end = half_time + margin;
    end.ceil() as u64 - start.floor() as u64 - 1
}

#[test]
fn test_find_error_margin() {
    assert_eq!(find_error_margin(7, 9), 4);
    assert_eq!(find_error_margin(15, 40), 8);
    assert_eq!(find_error_margin(30, 200), 9);
}

pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines();
    let next_parsed_line = &mut || {
        lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(str::parse)
            .collect()
    };
    let time_line: Result<Vec<u64>, _> = next_parsed_line();
    let time_line = time_line.expect("Error parsing time line");
    let distance_line: Result<Vec<u64>, _> = next_parsed_line();
    let distance_line = distance_line.expect("Error parsing distance line");

    time_line
        .iter()
        .zip(distance_line.iter())
        .map(|(time, distance)| find_error_margin(*time, *distance))
        .product()
}

#[test]
fn test_solution() {
    assert_eq!(
        solution(
            r#"Time:      7  15   30
Distance:  9  40  200
"#
        ),
        288
    );
}
