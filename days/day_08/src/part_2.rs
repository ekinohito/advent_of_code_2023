use std::collections::HashMap;

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub fn solution(input: &str) -> u64 {
    let mut dict = HashMap::new();
    let mut lines = input.lines();
    let lr_sequence = lines.next().unwrap();
    let lr_len = lr_sequence.len() as u64;
    lines.next().unwrap();
    let mut points = vec![];

    for line in lines {
        let (point, rest) = line.split_once('=').unwrap();
        let point = point.trim();
        let (next_left, next_right) = rest
            .trim_matches(|c| matches!(c, '(' | ')' | ' '))
            .split_once(',')
            .unwrap();
        let next_left = next_left.trim();
        let next_right = next_right.trim();
        dict.insert(point, (next_left, next_right));
        if point.ends_with('A') {
            points.push(point);
        }
    }

    let mut lcd = 1;

    for point in &points {
        let point = *point;
        let mut counter = 0;
        lr_sequence
            .chars()
            .enumerate()
            .cycle()
            .try_fold(point, &mut |acc: &str, (i, val)| {
                counter += 1;
                if acc.ends_with('Z') {
                    eprintln!("{i} -> {acc}")
                }
                dict.get(acc)
                    .map(|(next_left, next_right)| match val {
                        'L' => next_left,
                        'R' => next_right,
                        _ => panic!(),
                    })
                    .map(|next| (!next.ends_with('Z')).then_some(*next))
                    .unwrap()
            });
        eprintln!("{point} {counter} {} {}", counter / lr_len, counter % lr_len);
        lcd = lcd * counter / gcd(lcd, counter);
    }
    

    lcd
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#), 6)
}
