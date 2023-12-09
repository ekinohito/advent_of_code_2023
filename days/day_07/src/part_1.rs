use crate::structs::Hand;

pub fn universal_solution(input: &str, use_joker: bool) -> u64 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Hand::from_str(cards, bid.parse().unwrap(), use_joker).unwrap()
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.2)
        .sum()
}

pub fn solution(input: &str) -> u64 {
    universal_solution(input, false)
}

#[test]
fn test_solution() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(solution(input), 6440)
}
