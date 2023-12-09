use std::collections::HashMap;

pub fn solution(input: &str) -> u64{
    let mut dict = HashMap::new();
    let mut lines = input.lines();
    let lr_sequence = lines.next().unwrap();
    lines.next().unwrap();
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
    }
    let mut counter = 0;
    lr_sequence
        .chars()
        .cycle()
        .try_fold("AAA", &mut |acc: &str, val| {
            counter += 1;
            dict.get(acc)
                .map(|(next_left, next_right)| match val {
                    'L' => next_left,
                    'R' => next_right,
                    _ => panic!(),
                })
                .map(|next| (*next != "ZZZ").then_some(*next))
                .unwrap()
        });
    return counter;
}
