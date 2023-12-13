use crate::part_1::*;

pub fn solution(input: &str) -> u64 {
    input
        .lines()
        .map(parse_line)
        .map(|(marks, groups)| {
            let marks = vec![&marks[..]; 5].join(&Mark::Unknown);
            let groups = groups.repeat(5);
            eprint!("{marks:?} / {groups:?}");
            let result = possible_combinations(&marks[..], 0, &groups[..]) as u64;
            eprintln!("= {result}");
            result
        })
        .sum()
}

#[test]
fn test_solution() {
    assert_eq!(solution("???.### 1,1,3"), 1);
    assert_eq!(solution(".??..??...?##. 1,1,3"), 16384);
    assert_eq!(solution("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(solution("????.#...#... 4,1,1"), 16);
    assert_eq!(solution("????.######..#####. 1,6,5"), 2500);
    assert_eq!(solution("?###???????? 3,2,1"), 506250);
}
