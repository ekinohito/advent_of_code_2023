// #[derive(Clone, PartialEq)]
// enum Cell {
//     Empty,
//     Symbol,
//     Digit(u8),
// }
// use Cell::*;

use std::collections::HashSet;

pub fn solution(input: &str) -> u64 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = 0;
    let mut counted: HashSet<(usize, usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if !matches!(matrix[i][j], '0'..='9' | '.') {
                (i.saturating_sub(1)..=(i + 1).min(n - 1))
                    .flat_map(|i| (j.saturating_sub(1)..=(j + 1).min(m - 1)).map(move |j| (i, j)))
                    .for_each(|(i, j)| {
                        let mut start = j;
                        let mut end = j;
                        if !matrix[i][j].is_digit(10) {return;}
                        while start > 0 && matrix[i][start - 1].is_digit(10) {
                            start -= 1;
                        }
                        while end < m && matrix[i][end].is_digit(10) {
                            end += 1;
                        }
                        if start >= end {return;}
                        if !counted.insert((i, start, end)) {return}
                        let number = &matrix[i][start..end].iter().fold(0, |acc, val| acc * 10 + val.to_digit(10).unwrap() as u64);

                        result += number;
                    })
            }
        }
    }
    result
}
