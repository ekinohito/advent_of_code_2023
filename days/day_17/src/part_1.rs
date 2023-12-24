
use crate::{grid::Grid, position::Position};

pub fn solution(input: &str) -> u64 {
    let grid = Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    );
    let from = Position::new(0, 0);
    let to = Position::new(grid.n - 1, grid.m - 1);
    grid.minimize_loss(from, to)
}

#[test]
fn test_simple() {
    assert_eq!(
        solution(
            r#"213
311
334"#
        ),
        7
    );
}

#[test]
fn test_solution() {
    assert_eq!(
        solution(
            r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
        ),
        102
    );
}
