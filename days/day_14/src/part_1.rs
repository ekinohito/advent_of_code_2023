#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Empty,
    Round,
    Cube,
}

use std::collections::HashMap;

use Block::*;

#[derive(Debug)]
pub struct Grid {
    matrix: Vec<Vec<Block>>,
    n: usize,
    m: usize,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(matrix: Vec<Vec<Block>>) -> Self {
        let n = matrix.len();
        let m = matrix
            .first()
            .expect("Matrix must have at least one row")
            .len();
        Self { matrix, n, m }
    }

    pub fn calc_tilt(&self) -> usize {
        let Self { matrix, n, m } = self;
        let mut obstacles = vec![0; *m];
        let mut result = 0;
        for i in 0..*n {
            for j in 0..*m {
                match matrix[i][j] {
                    Empty => {}
                    Cube => obstacles[j] = i + 1,
                    Round => {
                        result += n - obstacles[j];
                        obstacles[j] += 1
                    }
                }
            }
        }
        result
    }

    pub fn tilt_north(&mut self) {
        let Self { matrix, n, m } = self;
        let mut obstacles = vec![0; *m];
        for i in 0..*n {
            for j in 0..*m {
                match matrix[i][j] {
                    Empty => {}
                    Cube => obstacles[j] = i + 1,
                    Round => {
                        matrix[i][j] = Empty;
                        matrix[obstacles[j]][j] = Round;
                        obstacles[j] += 1;
                    }
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        let Self { matrix, n, m } = self;
        let mut obstacles = vec![0; *n];
        for j in 0..*m {
            for i in 0..*n {
                match matrix[i][j] {
                    Empty => {}
                    Cube => obstacles[i] = j + 1,
                    Round => {
                        matrix[i][j] = Empty;
                        matrix[i][obstacles[i]] = Round;
                        obstacles[i] += 1;
                    }
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        let Self { matrix, n, m } = self;
        let mut obstacles = vec![*n - 1; *m];
        for i in (0..*n).rev() {
            for j in 0..*m {
                match matrix[i][j] {
                    Empty => {}
                    Cube => obstacles[j] = i.saturating_sub(1),
                    Round => {
                        matrix[i][j] = Empty;
                        matrix[obstacles[j]][j] = Round;
                        obstacles[j] = obstacles[j].saturating_sub(1);
                    }
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        let Self { matrix, n, m } = self;
        let mut obstacles = vec![*m - 1; *n];
        for j in (0..*m).rev() {
            for i in 0..*n {
                match matrix[i][j] {
                    Empty => {}
                    Cube => obstacles[i] = j.saturating_sub(1),
                    Round => {
                        matrix[i][j] = Empty;
                        matrix[i][obstacles[i]] = Round;
                        obstacles[i] = obstacles[i].saturating_sub(1);
                    }
                }
            }
        }
    }

    pub fn debug(&self) {
        for row in &self.matrix {
            for block in row {
                print!(
                    "{}",
                    match block {
                        Empty => '.',
                        Round => 'O',
                        Cube => '#',
                    }
                )
            }
            println!();
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn spin_cycles(&mut self, amount: usize) {
        let mut visited = HashMap::new();
        for i in 0..amount {
            let key = self.get_key();
            if let Some(p) = visited.get(&key) {
                let period = i - p;
                let remaining = amount - i - 1;
                let i = i + (remaining / period) * period;
                for _ in i..amount {
                    self.cycle();
                }
                return;
                // println!("AMOGUS {} -> {}", p, i);
                // return 
            } else {
                visited.insert(key, i);
            }
            self.cycle();
        }
    }

    pub fn evaluate(&self) -> usize {
        let mut result = 0;
        for (i, row) in self.matrix.iter().enumerate() {
            for block in row {
                if *block == Round {
                    result += self.n - i;
                }
            }
        }
        result
    }

    pub fn get_key(&self) -> Box<[(usize, usize)]> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, block)| **block == Round)
                    .map(move |(j, _)| (i, j))
            })
            .collect()
    }
}

pub fn parse_input(input: &str) -> Grid {
    Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Empty,
                        'O' => Round,
                        '#' => Cube,
                        _ => panic!("Contains unexpected characters"),
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn solution(input: &str) -> u64 {
    let grid = parse_input(input);
    grid.calc_tilt() as u64
}
