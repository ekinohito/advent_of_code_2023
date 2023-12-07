use crate::parsing::{parse_range_map, parse_seeds_range, split_paragraphs};

pub fn solution(input: &str) -> u64 {
    let mut paragraphs = split_paragraphs(input);
    let seeds = parse_seeds_range(paragraphs.next().unwrap());
    let range_maps: Vec<_> = paragraphs.map(parse_range_map).collect();
    seeds
        .iter()
        .map(|(start, len)| {
            *range_maps
                .iter()
                .fold(vec![*start..=start + len - 1], |acc, range_map| {
                    range_map.get_ranges(acc)
                })
                .iter()
                .map(|e| e.start())
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[test]
fn feature() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    dbg!(solution(&input));
}
