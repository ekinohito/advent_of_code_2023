use crate::part_1::*;

pub fn solution(input: &str) -> u64 {
    let mut grid = parse_input(input);
    // grid.debug();
    // println!();
    grid.spin_cycles(1_000_000_000);
    // grid.debug();

    grid.evaluate() as u64
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#), 64);
}
