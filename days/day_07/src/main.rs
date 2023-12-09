const INPUT: &str = include_str!("input.txt");

mod structs;
mod part_1;
mod part_2;


fn main() {
    println!("part 1: {}", part_1::solution(INPUT));
    println!("part 2: {}", part_2::solution(INPUT));
}
