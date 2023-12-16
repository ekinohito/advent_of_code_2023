#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Empty,
    Mirror,
    BackMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

use std::mem;

use Block::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

pub struct Beam {
    pub position: (usize, usize),
    pub direction: Direction,
}

pub struct Grid {
    matrix: Vec<Vec<Block>>,
    n: usize,
    m: usize,
}

impl Grid {
    pub fn new(matrix: Vec<Vec<Block>>) -> Self {
        let n = matrix.len();
        let m = matrix.get(0).expect("no lines").len();
        Self { matrix, n, m }
    }

    pub fn simulate_light(&self, start: Beam) -> u64 {
        let Self { matrix, n, m } = self;
        let mut processing = vec![start];
        let mut next_processing = vec![];
        let mut visited = vec![vec![[false; 4]; *m]; *n];
        let mut result = 0;
        while !processing.is_empty() {
            for beam in &processing {
                let Beam {
                    position: (i, j),
                    direction,
                } = *beam;
                if visited[i][j] == [false; 4] {
                    result += 1;
                }
                visited[i][j][direction as usize] = true;
                let block = matrix[i][j];
                let next_directions = match (block, direction) {
                    (Empty, North) => [North].as_slice(),
                    (Empty, East) => [East].as_slice(),
                    (Empty, South) => [South].as_slice(),
                    (Empty, West) => [West].as_slice(),

                    (Mirror, North) => [East].as_slice(),
                    (Mirror, West) => [South].as_slice(),
                    (Mirror, South) => [West].as_slice(),
                    (Mirror, East) => [North].as_slice(),

                    (BackMirror, North) => [West].as_slice(),
                    (BackMirror, West) => [North].as_slice(),
                    (BackMirror, South) => [East].as_slice(),
                    (BackMirror, East) => [South].as_slice(),

                    (VerticalSplitter, North) => [North].as_slice(),
                    (VerticalSplitter, South) => [South].as_slice(),
                    (VerticalSplitter, East | West) => [North, South].as_slice(),
                    (HorizontalSplitter, North | South) => [East, West].as_slice(),
                    (HorizontalSplitter, West) => [West].as_slice(),
                    (HorizontalSplitter, East) => [East].as_slice(),
                };

                for next in next_directions {
                    let (di, dj) = match next {
                        North => (-1, 0),
                        East => (0, 1),
                        South => (1, 0),
                        West => (0, -1),
                    };
                    let (Some(next_i), Some(next_j)) =
                        (i.checked_add_signed(di), j.checked_add_signed(dj))
                    else {
                        continue;
                    };
                    if next_i >= *n || next_j >= *m {
                        continue;
                    };
                    if visited[next_i][next_j][*next as usize] {
                        continue;
                    };
                    next_processing.push(Beam { position: (next_i, next_j), direction: *next });
                }
            }

            mem::swap(&mut processing, &mut next_processing);
            next_processing.clear();
        }
        // visited.iter().for_each(|row| {
        //     row.iter().for_each(|cell| {
        //         print!("{}", if *cell == [false; 4] {'.'} else {'#'})
        //     });
        //     println!();
        // });
        result
    }

    pub fn maximize_energy(&self) -> u64 {
        let n = self.n;
        let m = self.n;

        let north_row = (0..m).map(|j| Beam {position: (0, j), direction: South});
        let east_col = (0..n).map(|i| Beam {position: (i, m - 1), direction: West});
        let south_row = (0..m).map(|j| Beam {position: (n - 1, j), direction: North});
        let west_col = (0..n).map(|i| Beam {position: (i, 0), direction: East});

        north_row.chain(east_col).chain(south_row).chain(west_col).map(|beam| self.simulate_light(beam)).max().unwrap()
    }
}

fn parse_char(ch: char) -> Block {
    match ch {
        '.' => Empty,
        '/' => Mirror,
        '\\' => BackMirror,
        '|' => VerticalSplitter,
        '-' => HorizontalSplitter,
        _ => panic!(),
    }
}

pub fn parse_input(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|line| line.chars().map(parse_char).collect())
            .collect(),
    )
}

pub fn solution(input: &str) -> u64 {
    let grid = parse_input(input);
    grid.simulate_light(Beam { position: (0, 0), direction: East })
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#), 46)
}
