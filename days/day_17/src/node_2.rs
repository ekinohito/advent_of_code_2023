use crate::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Duration {
    ZeroLeft,
    OneLeft,
    TwoLeft,
    ThreeLeft,
    FourLeft,
    FiveLeft,
    SixLeft,
}

impl Duration {
    pub fn one_less(&self) -> Option<Duration> {
        match self {
            Duration::ZeroLeft => None,
            Duration::OneLeft => Some(Self::ZeroLeft),
            Duration::TwoLeft => Some(Self::OneLeft),
            Duration::ThreeLeft => Some(Self::TwoLeft),
            Duration::FourLeft => Some(Self::ThreeLeft),
            Duration::FiveLeft => Some(Self::FourLeft),
            Duration::SixLeft => Some(Self::FiveLeft),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeInfo {
    pub pos: Position,
    pub dir: Direction,
    pub dur: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    pub heat: u64,
    pub info: NodeInfo,
}

impl Node {
    pub fn new(heat: u64, pos: Position, dir: Direction, dur: Duration) -> Self {
        Self {
            heat,
            info: NodeInfo { pos, dir, dur },
        }
    }

    pub fn from_pos(pos: Position) -> [Self; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .map(|dir| Node::new(0, pos, dir, Duration::ZeroLeft))
    }

    pub fn connected_to(&self, grid: &Grid) -> [Option<Node>; 3] {
        [self.info.dir.turn_left(), self.info.dir.turn_right(), self.info.dir].map(|direction| {
            let (pos, heat, dur) = if direction == self.info.dir {
                let pos = grid.next_in_direction(self.info.pos, direction)?;
                let dur = self.info.dur.one_less()?;
                let heat = self.heat + grid[pos] as u64;
                (pos, heat, dur)
            } else {
                let mut heat = self.heat;
                let mut pos = self.info.pos;
                for _ in 0..4 {
                    pos = grid.next_in_direction(pos, direction)?;
                    heat += grid[pos] as u64;
                }
                let dur = Duration::SixLeft;
                (pos, heat, dur)
            };
            Some(Node::new(heat, pos, direction, dur))
        })
    }
}
