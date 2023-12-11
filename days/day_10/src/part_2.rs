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
    let mut masks = vec![vec![]; blocks.len()];

    while left.0 != right.0 || left.1 != right.1 {
        for (i, j, side) in [&mut left, &mut right] {
            // dbg!(&i, &j, &side);
            masks[*i].push(*j);

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
    masks.iter_mut().enumerate().for_each(|(i, mask)| {
        let mut start = None;
        let mut edges = 0;
        mask.sort();
        mask.retain(|elem| {
            let block = blocks[i][*elem];
            if block == Pipe(Top, Bottom)  {
                return true;
            }
            if block == Pipe(Right, Left) {
                return false;
            }

            let Some(start_block) = start else {
                start = Some(block);
                return edges % 2 == 1;
            };
            start = None;
            if i == 112 {
                // dbg!(&i, &elem, &start_block, &block, &edges);
            }
            return match (start_block, block) {
                (Block::TR, Block::LT) | (Block::RB, Block::BL) => false,
                (Block::TR, Block::BL) | (Block::RB, Block::LT) => {
                    edges += 1;
                    edges % 2 == 0
                }
                _ => {println!("{:?}", (i, elem, start_block, block, edges)); false},
            }
        });
    });
    masks.iter().map(|mask| {
        mask.chunks_exact(2).map(|chunk| {
            let [start, end] = chunk else {unreachable!()};
            (end - start - 1) as u64
        }).sum::<u64>()
    }).sum() 
}