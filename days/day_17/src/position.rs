use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

impl Position {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
}

impl<T> Index<Position> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self[index.i][index.j]
    }
}

impl<T> IndexMut<Position> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.i][index.j]
    }
}
