use std::str::FromStr;
use crate::structs::{RangeMap, RangeMapDto};

pub fn split_paragraphs(text: &str) -> impl Iterator<Item = &str> {
    text.split("\n\n")
}

pub fn parse_range_map(text: &str) -> RangeMap {
    text.lines().skip(1).map(|line| {
        let mut result = line.split_whitespace().map(u64::from_str).map(Result::unwrap);
        let to = result.next().unwrap();
        let from = result.next().unwrap();
        let len = result.next().unwrap();
        RangeMapDto::new(from, to, len)
    }).collect()
}

pub fn parse_seeds(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_whitespace().skip(1).map(u64::from_str).map(Result::unwrap)
}

pub fn parse_seeds_range(line: &str) -> Vec<(u64, u64)> {
    let mut result = vec![];
    let mut iter = line.split_whitespace().skip(1).map(u64::from_str).map(Result::unwrap);
    while let Some(start) = iter.next() {
        let len = iter.next().unwrap();
        result.push((start, len));
    }
    result
}
