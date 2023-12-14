#[derive(Debug)]
pub struct Grid {
    matrix: Vec<Vec<bool>>,
    n: usize,
    m: usize,
}

impl Grid {
    pub fn new(matrix: Vec<Vec<bool>>) -> Self {
        let n = matrix.len();
        let m = matrix
            .first()
            .expect("Grid must contain at least one row")
            .len();
        Self { matrix, n, m }
    }

    fn is_v_symmetric(&self, i: usize) -> bool {
        let right_side = &self.matrix[i..];
        let left_side = &self.matrix[..i];
        left_side
            .iter()
            .rev()
            .zip(right_side.iter())
            .all(|(left, right)| left == right)
    }

    fn is_h_symmetric(&self, j: usize) -> bool {
        self.matrix.iter().all(|line| {
            let right_side = &line[j..];
            let left_side = &line[..j];
            left_side
                .iter()
                .rev()
                .zip(right_side.iter())
                .all(|(left, right)| left == right)
        })
    }

    fn find_v_symmetry(&self) -> Option<usize> {
        (1..self.n).find(|i| self.is_v_symmetric(*i))
    }

    fn find_h_symmetry(&self) -> Option<usize> {
        (1..self.m).find(|j| self.is_h_symmetric(*j))
    }

    pub fn find_symmetry(&self) -> Option<usize> {
        self.find_v_symmetry()
            .map(|i| 100 * i)
            .or_else(|| self.find_h_symmetry())
    }

    pub fn is_almost_v_symmetric(&self, i: usize) -> bool {
        let right_side = &self.matrix[i..];
        let left_side = &self.matrix[..i];
        let mut res = left_side
            .iter()
            .rev()
            .zip(right_side.iter())
            .map(|(left, right)| {
                left.iter()
                    .zip(right.iter())
                    .filter(|(left, right)| left != right)
                    .take(2)
                    .count()
            })
            .filter(|matches| *matches != 0)
            .take(2);
        let x = res.next();
        let y = res.next();
        x == Some(1) && y == None
    }

    pub fn is_almost_h_symmetric(&self, j: usize) -> bool {
        let mut res = self.matrix.iter().map(|line| {
            let right_side = &line[j..];
            let left_side = &line[..j];
            left_side
                .iter()
                .rev()
                .zip(right_side.iter())
                .filter(|(left, right)| left != right)
                .take(2)
                .count()
        })
        .filter(|matches| *matches != 0)
        .take(2);
        res.next() == Some(1) && res.next() == None
    }

    fn find_almost_v_symmetry(&self) -> Option<usize> {
        (1..self.n).find(|i| self.is_almost_v_symmetric(*i))
    }

    fn find_almost_h_symmetry(&self) -> Option<usize> {
        (1..self.m).find(|j| self.is_almost_h_symmetric(*j))
    }

    pub fn find_almost_symmetry(&self) -> Option<usize> {
        self.find_almost_v_symmetry()
            .map(|i| 100 * i)
            .or_else(|| self.find_almost_h_symmetry())
    }
}

pub fn parse_input(input: &str) -> Box<[Grid]> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect())
                .collect()
        })
        .map(Grid::new)
        .collect()
}

pub fn solution(input: &str) -> u64 {
    let grids = parse_input(input);
    // dbg!(&grids);
    grids
        .iter()
        .map(Grid::find_symmetry)
        .map(Option::unwrap)
        .sum::<usize>() as u64
}

#[test]
fn test_solution() {
    assert_eq!(
        solution(
            r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
        ),
        405
    )
}
