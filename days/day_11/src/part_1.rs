#[derive(Debug)]
pub struct Pos {
    i: usize,
    j: usize,
}

pub fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(move |(j, char)| (char == '#').then_some(Pos{i, j}))
        })
        .collect()
}

pub fn expand(galaxies: &mut Vec<Pos>, k: usize) {
    galaxies.sort_by_key(|galaxy| galaxy.i);
    let mut current_i = galaxies[0].i;
    let mut prev_i = galaxies[0].i;
    for galaxy in galaxies.iter_mut() {
        current_i += ((galaxy.i - prev_i) * k).saturating_sub(k - 1);
        prev_i = galaxy.i;
        galaxy.i = current_i;
    }

    galaxies.sort_by_key(|galaxy| galaxy.j);
    let mut current_j = galaxies[0].j;
    let mut prev_j = galaxies[0].j;
    for galaxy in galaxies.iter_mut() {
        current_j += ((galaxy.j - prev_j) * k).saturating_sub(k - 1);
        prev_j = galaxy.j;
        galaxy.j = current_j;
    }
}

pub fn distance(galaxy_1: &Pos, galaxy_2: &Pos) -> usize {
    galaxy_1.i.abs_diff(galaxy_2.i) + galaxy_1.j.abs_diff(galaxy_2.j)
}

pub fn solution(input: &str) -> u64 {
    let mut galaxies = parse_input(input);
    expand(&mut galaxies, 2);
    let mut result = 0;
    for galaxy_1_index in 0..galaxies.len() {
        for galaxy_2_index in galaxy_1_index + 1..galaxies.len() {
            result += distance(&galaxies[galaxy_1_index], &galaxies[galaxy_2_index]);
        }
    }
    result as u64
}
