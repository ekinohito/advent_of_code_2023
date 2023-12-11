#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub use Side::*;

impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Top => Bottom,
            Right => Left,
            Bottom => Top,
            Left => Right,
        }
    }

    pub fn to_direction(&self) -> (isize, isize) {
        match self {
            Top => (-1, 0),
            Right => (0, 1),
            Bottom => (1, 0),
            Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Ground,
    Start,
    Pipe(Side, Side),
    MainPipe(Side, Side),
}

impl Block {
    pub const TB: Self = Pipe(Top, Bottom);
    pub const RL: Self = Pipe(Right, Left);
    pub const TR: Self = Pipe(Top, Right);
    pub const LT: Self = Pipe(Left, Top);
    pub const BL: Self = Pipe(Bottom, Left);
    pub const RB: Self = Pipe(Right, Bottom);

}

pub use Block::*;


pub fn solution(input: &str) -> u64 {
    let blocks: Vec<Vec<Block>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Ground,
                    'S' => Start,
                    '|' => Pipe(Top, Bottom),
                    '-' => Pipe(Right, Left),
                    'L' => Pipe(Top, Right),
                    'J' => Pipe(Left, Top),
                    '7' => Pipe(Bottom, Left),
                    'F' => Pipe(Right, Bottom),
                    _ => panic!("Unexpected character"),
                })
                .collect()
        })
        .collect();
    let (start_i, start_j) = blocks
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|block| *block == Start).map(|j| (i, j)))
        .expect("No starting block found");

    let mut adjacents = [(-1, 0, Bottom), (0, 1, Left), (1, 0, Top), (0, -1, Right)]
        .iter()
        .filter_map(|(di, dj, side)| {
            let i = start_i.checked_add_signed(*di)?;
            let j = start_j.checked_add_signed(*dj)?;
            let block = blocks.get(i)?.get(j)?;
            let Pipe(first, second) = block else {
                return None;
            };
            if side == first || side == second {
                return Some((i, j, *side));
            }
            None
        });

    let mut left = adjacents.next().expect("No adjacent pipes");
    let mut right = adjacents.next().expect("Only one adjacent pipe");
    assert_eq!(adjacents.next(), None, "Too many adjacent pipes");
    let mut counter = 1;

    while left.0 != right.0 || left.1 != right.1 {
        counter += 1;
        for (i, j, side) in [&mut left, &mut right] {
            // dbg!(&i, &j, &side);
            let Pipe(first, second) = blocks[*i][*j] else {panic!("Lost the pipe")};
            let next_side = if *side == first {
                second
            } else {
                first
            };
            let (di, dj) = next_side.to_direction();
            *i = i.checked_add_signed(di).unwrap();
            *j = j.checked_add_signed(dj).unwrap();
            *side = next_side.opposite();
        }
    }
    counter
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#), 8);
}
