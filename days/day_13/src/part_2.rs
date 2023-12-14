use crate::part_1::*;

pub fn solution(input: &str) -> u64 {
    let grids = parse_input(input);
    grids
        .iter()
        .map(Grid::find_almost_symmetry)
        .map(|o: Option<usize>| o.unwrap())
        .sum::<usize>() as u64
}

#[test]
fn test_solution() {
    assert_eq!(
        solution(
            r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
        ),
        400
    )
}
