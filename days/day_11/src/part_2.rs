use crate::part_1::*;

pub fn solution(input: &str) -> u64 {
    let mut galaxies = parse_input(input);
    expand(&mut galaxies, 1_000_000);
    let mut result = 0;
    for galaxy_1_index in 0..galaxies.len() {
        for galaxy_2_index in galaxy_1_index + 1..galaxies.len() {
            result += distance(&galaxies[galaxy_1_index], &galaxies[galaxy_2_index]);
        }
    }
    result as u64
}