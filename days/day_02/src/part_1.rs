use crate::game::Game;

const RED: u64 = 12;
const GREEN: u64 = 13;
const BLUE: u64 = 14;

fn process_line(line: &str) -> Option<u64> {
    let game: Game = line.parse().expect("Parsing error");
    game.is_possible(RED, GREEN, BLUE).then_some(game.id)
}

pub fn solution(input: &str) -> u64 {
    input.lines().map(process_line).flatten().sum()
}
