use crate::part_1::universal_solution;

pub fn solution(input: &str) -> u64 {
    universal_solution(input, true)
}

#[test]
fn test_solution() {
    let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    assert_eq!(solution(input), 5905)
}
