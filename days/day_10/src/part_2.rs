use crate::part_1::*;

pub fn solution(input: &str) -> u64 {
    let mut blocks: Vec<Vec<Block>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Ground,
                    'S' => Start,
                    '|' => Block::TB,
                    '-' => Block::RL,
                    'L' => Block::TR,
                    'J' => Block::LT,
                    '7' => Block::BL,
                    'F' => Block::RB,
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

    dbg!(&start_i, &start_j);

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
    blocks[start_i][start_j] = Pipe(left.2.opposite(), right.2.opposite());
    dbg!(&blocks[start_i][start_j]);

    while left.0 != right.0 || left.1 != right.1 {
        for (i, j, side) in [&mut left, &mut right] {
            let Pipe(first, second) = blocks[*i][*j] else {
                panic!("Lost the pipe")
            };
            blocks[*i][*j] = MainPipe(first, second);
            let next_side = if *side == first { second } else { first };
            let (di, dj) = next_side.to_direction();
            *i = i.checked_add_signed(di).unwrap();
            *j = j.checked_add_signed(dj).unwrap();
            *side = next_side.opposite();
        }
    }

    blocks.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|block| {
            *block = if let MainPipe(s1, s2) = block {
                Pipe(*s1, *s2)
            } else {
                Ground
            }
        })
    });


    let mut result = 0;
    for row in &mut blocks {
        let mut within_loop = false;
        let mut start = None;
        for block in row {
            match block {
                Ground => {
                    if within_loop {
                        result += 1;
                        *block = Start;
                    }
                },
                Pipe(Top, Bottom) => {
                    within_loop = !within_loop;
                }
                Pipe(Right, Left) => {},
                Pipe(side_1, side_2) => {
                    let direction = if *side_1 == Top || *side_2 == Top {Top} else {Bottom};
                    if let Some(start_direction) = start {
                        if start_direction != direction {
                            within_loop = !within_loop;
                        }
                        start = None;
                    } else {
                        start = Some(direction);
                    }
                },
                _ => unreachable!()
            }
        }
    }
    blocks.iter().for_each(|row| {row.iter().for_each(|b| print!("{}", match *b {
        Ground => '.',
        Block::TB => '┃',
        Block::RL => '━',
        Block::TR => '┗',
        Block::RB => '┏',
        Block::BL => '┓',
        Block::LT => '┛',
        Start => 'x',
        _ => 'U',
    })); println!()});

    result
}
