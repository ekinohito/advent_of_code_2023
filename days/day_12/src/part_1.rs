#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Unknown,
    Damaged,
    Operational,
}

use Mark::*;

pub fn parse_line(line: &str) -> (Box<[Mark]>, Box<[usize]>) {
    let (marks, groups) = line.split_once(' ').unwrap();

    let marks = marks
        .chars()
        .map(|ch| match ch {
            '?' => Unknown,
            '#' => Damaged,
            '.' => Operational,
            _ => panic!(),
        })
        .collect();
    let groups = groups.split(',').map(str::parse).map(Result::unwrap).collect();
    (marks, groups)
}

pub fn possible_combinations(marks: &[Mark], current_group: usize, groups: &[usize]) -> usize {
    let Some(current_mark) = marks.first().cloned() else {
        let [group] = groups else {
            return if current_group == 0 && groups.is_empty() { 1 } else { 0 };
        };
        return if current_group == *group { 1 } else { 0 };
    };
    let current_group_capacity = groups.first().cloned().unwrap_or(0);
    if current_group > current_group_capacity {
        return 0;
    };

    let mut result = 0;
    if let Damaged | Unknown = current_mark {
        if current_group < current_group_capacity {
            result += possible_combinations(&marks[1..], current_group + 1, groups)
        };
    }
    if let Operational | Unknown = current_mark {
        if current_group == 0 {
            result += possible_combinations(&marks[1..], 0, groups)
        } else if current_group == current_group_capacity {
            result += possible_combinations(&marks[1..], 0, &groups[1..])
        }
    }

    result
}

#[test]
fn test_solution() {
    assert_eq!(solution("#.#.### 1,1,3"), 1);
    assert_eq!(solution("#.#.### 1,2,3"), 0);

    assert_eq!(solution("???.### 1,1,3"), 1);
    assert_eq!(solution(".??..??...?##. 1,1,3"), 4);
    assert_eq!(solution("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(solution("????.#...#... 4,1,1"), 1);
    assert_eq!(solution("????.######..#####. 1,6,5"), 4);
    assert_eq!(solution("??? 1"), 3);
    assert_eq!(solution("?.? 1,1"), 1);
    assert_eq!(solution("??????? 2,1"), 10);
    assert_eq!(solution("?###???????? 3,2,1"), 10);
}

pub fn solution(input: &str) -> u64 {
    input
        .lines()
        .map(parse_line)
        .map(|(marks, groups)| possible_combinations(&marks[..], 0, &groups[..]) as u64)
        .sum()
}
