const INPUT: &str = include_str!("input.txt");

mod part_1;
mod part_2;
mod grid;
mod position;
mod direction;
mod beam;
mod node;


fn main() {
    println!("part 1: {}", part_1::solution(INPUT));
    println!("part 2: {}", part_2::solution(INPUT));
}
