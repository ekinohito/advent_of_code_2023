use crate::{direction::Direction, grid::Grid, position::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Duration {
    TwoLeft,
    OneLeft,
    ZeroLeft,
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
        .map(|dir| Node::new(0, pos, dir, Duration::TwoLeft))
    }

    pub fn connected_to(&self, grid: &Grid) -> [Option<Node>; 3] {
        let Some(pos) = grid.next_in_direction(self.info.pos, self.info.dir) else {
            return [None; 3];
        };
        let heat = self.heat + grid[pos] as u64;
        let turn_left = Some(Node::new(
            heat,
            pos,
            self.info.dir.turn_left(),
            Duration::TwoLeft,
        ));
        let turn_right = Some(Node::new(
            heat,
            pos,
            self.info.dir.turn_right(),
            Duration::TwoLeft,
        ));
        let dont_turn = match self.info.dur {
            Duration::TwoLeft => Some(Node::new(heat, pos, self.info.dir, Duration::OneLeft)),
            Duration::OneLeft => Some(Node::new(heat, pos, self.info.dir, Duration::ZeroLeft)),
            Duration::ZeroLeft => None,
        };
        [turn_left, dont_turn, turn_right]
    }

    // pub fn connected_from(&self, grid: &Grid) -> [Option<Node>; 3] {
    //     let turn_left = self.info.dir.turn_left();
    //     let turn_right = self.info.dir.turn_right();
    //     let opposite = self.info.dir.opposite();
    //     [turn_right, opposite, turn_left].map(|direction| {
    //         let pos = grid.next_in_direction(self.info.pos, direction)?;
    //         if direction == opposite {
    //             match self.info.dur {
    //                 Duration::TwoLeft => None,
    //                 Duration::OneLeft => Some(Node::new(pos, self.info.dir, Duration::TwoLeft)),
    //                 Duration::ZeroLeft => Some(Node::new(pos, self.info.dir, Duration::OneLeft)),
    //             }
    //         } else {
    //             Some(Node::new(pos, direction.opposite(), Duration::TwoLeft))
    //         }
    //     })
    // }
}
