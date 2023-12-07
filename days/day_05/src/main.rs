const INPUT: &str = include_str!("input.txt");

mod structs;
mod parsing;
mod part_1;
mod part_2;


fn main() {
    println!("part 1: {}", part_1::solution(INPUT));
    // println!("part 2: {}", part_2::solution(INPUT));
    let x = &[1, 2, 3][2..2];
    println!("{}", x.len())
}
