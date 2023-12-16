use crate::part_1::parse_input;

pub fn solution(input: &str) -> u64 {
    let grid = parse_input(input);
    grid.maximize_energy()
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#), 51)
}
