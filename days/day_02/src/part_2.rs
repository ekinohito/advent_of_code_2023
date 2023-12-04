use crate::game::Game;

fn process_line(line: &str) -> u64 {
    let game: Game = line.parse().expect("Parsing error");
    game.power()
}

pub fn solution(input: &str) -> u64 {
    input.lines().map(process_line).sum()
}
