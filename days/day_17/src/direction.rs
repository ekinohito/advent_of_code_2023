#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn delta(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }
}