use std::ops::Index;

use crate::{beam::Beam, direction::Direction, position::Position};

#[derive(Debug)]
pub struct Grid {
    matrix: Vec<Vec<u8>>,
    pub n: usize,
    pub m: usize,
}

impl Grid {
    pub fn new(matrix: Vec<Vec<u8>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        Self { matrix, n, m }
    }

    pub fn next_in_direction(&self, from: Beam, to: Direction) -> Option<Beam> {
        if from.dir == to.opposite() || from.dir == to && from.dur == 0 {
            return None;
        }
        let (di, dj) = to.delta();
        let i = from.pos.i.checked_add_signed(di)?;
        let j = from.pos.j.checked_add_signed(dj)?;
        if i >= self.n || j >= self.m {
            return None;
        }
        let dur = if from.dir == to { from.dur - 1 } else { 2 };
        Some(Beam::new(Position::new(i, j), to, dur))
    }
}

impl Index<Position> for Grid {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        &self.matrix[index]
    }
}
