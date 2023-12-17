#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
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

    pub fn delta(&self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    i: usize,
    j: usize,
    dir: Direction,
    dur: usize,
}

struct Cache {
    inner: Vec<Vec<[[Option<usize>; 4]; 4]>>,
}

impl Cache {
    pub fn new(n: usize, m: usize) -> Self {
        let inner = vec![vec![[[None; 4]; 4]; m]; n];
        Self { inner }
    }

    pub fn get(&self, key: Beam) -> Option<usize> {
        self.inner[key.i][key.j][key.dir as usize][key.dur]
    }

    pub fn insert(&mut self, key: Beam, value: usize) {
        self.inner[key.i][key.j][key.dir as usize][key.dur] = Some(value);
    }
}

pub fn minimize_loss(grid: &Vec<Vec<u8>>) -> usize {
    fn inner(grid: &Vec<Vec<u8>>, cache: &mut Cache, from: Beam) -> usize {
        if let Some(result) = cache.get(from) {
            return result;
        }
        cache.insert(from, usize::MAX >> 8);
        let Beam { i, j, dir, dur } = from;
        let mut result = usize::MAX;
        for direction in [North, East, South, West] {
            if direction == dir.opposite() || direction == dir && dur == 0 {
                continue;
            }
            let (di, dj) = direction.delta();
            let Some((i, j)) = i.checked_add_signed(di).zip(j.checked_add_signed(dj)) else {
                continue;
            };
            if i >= grid.len() || j >= grid[0].len() {
                continue;
            }
            let dur = if direction == dir { dur - 1 } else { 2 };
            let dir = direction;
            result = result.min(inner(grid, cache, Beam { i, j, dir, dur }))
        }
        result += grid[i][j] as usize;
        cache.insert(from, result);
        result
    }
    let mut cache = Cache::new(grid.len(), grid[0].len());
    cache.inner[grid.len() - 1][grid[0].len() - 1] = [[Some(0); 4]; 4];
    inner(grid, &mut cache, Beam { i: 0, j: 0, dir: North, dur: 3 })
}

pub fn solution(input: &str) -> u64 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    minimize_loss(&grid) as u64
}

#[test]
fn test_solution() {
    assert_eq!(solution(r#"2413432311323
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
4322674655533"#), 102);
}
