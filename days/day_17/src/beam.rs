use crate::{direction::Direction, position::Position};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Beam {
    pub pos: Position,
    pub dir: Direction,
    pub dur: u8,
}

impl Beam {
    pub fn new(pos: Position, dir: Direction, dur: u8) -> Self {
        Self { pos, dir, dur }
    }
}
