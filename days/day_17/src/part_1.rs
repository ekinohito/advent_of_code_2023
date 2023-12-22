use std::collections::HashMap;

use crate::beam::Beam;
use crate::direction::Direction::*;
use crate::{grid::Grid, position::Position};

pub fn minimize_loss(grid: &Grid, from: Position, to: Position) -> usize {
    let mut visited = vec![vec![false; grid.m]; grid.n];
    let mut cache = HashMap::new();

    fn local_min(
        grid: &Grid,
        from: Beam,
        to: Position,
        visited: &mut Vec<Vec<bool>>,
        cache: &mut HashMap<Beam, Option<usize>>,
    ) -> Option<usize> {
        if let Some(&cached) = cache.get(&from) {
            return cached;
        }
        if from.pos == to {
            return Some(grid[from.pos] as usize);
        }
        visited[from.pos] = true;

        let mut result = usize::MAX;
        for direction in [North, East, South, West] {
            let Some(next) = grid.next_in_direction(from, direction) else {
                continue;
            };
            if visited[next.pos] {
                continue;
            };
            let Some(local) = local_min(grid, next, to, visited, cache) else {
                continue;
            };
            result = result.min(local);
        }

        visited[from.pos] = false;
        if result == usize::MAX {
            cache.insert(from, None);
            return None;
        }
        let result = Some(result + grid[from.pos] as usize);
        cache.insert(from, result);
        result
    }

    local_min(
        grid,
        Beam::new(Position::new(0, 0), North, 0),
        to,
        &mut visited,
        &mut cache,
    )
    .unwrap()
        - grid[from] as usize
}

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
    minimize_loss(&grid, from, to) as u64
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
