use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::utils::{add_vecs, FnvHash, FnvHashMap};

static DELTAS: &[[i16; 2]] = &[
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

struct ElfState {
    is_active: bool,
    pos: [i16; 2],
}

pub fn solve(input_path: &str, turn_limit: Option<usize>) -> (usize, FnvHashMap<[i16; 2], usize>) {
    let mut elves = parse_elves(input_path);
    let mut elf_positions = HashMap::with_capacity_and_hasher(elves.len(), FnvHash);
    for (elf_num, elf_state) in elves.iter().enumerate() {
        elf_positions.insert(elf_state.pos, elf_num);
    }
    let mut active_elves: Vec<usize> = (0..elves.len()).collect();

    // (ax, delta)
    let mut directions: [(usize, i16); 4] = [
        (1, -1), // North
        (1, 1),  // Sounth
        (0, -1), // West
        (0, 1),  // East
    ];
    let mut times_proposed: FnvHashMap<[i16; 2], usize> = HashMap::with_hasher(FnvHash);
    let mut proposed_moves = Vec::new();

    let mut turn_counter = 0;

    loop {
        proposed_moves.clear();
        times_proposed.clear();

        let mut i = 0;
        while i < active_elves.len() {
            let elf_num = active_elves[i];
            // println!("i: {}, elf {}", i, elf_num);
            let mut mask = [[false; 3]; 2];

            for delta in DELTAS {
                let p = add_vecs(&elves[elf_num].pos, delta);
                if let Some(&other_elf) = elf_positions.get(&p) {
                    if !elves[other_elf].is_active {
                        elves[other_elf].is_active = true;
                        active_elves.push(other_elf);
                    }
                    mask[0][(1 + delta[0]) as usize] = true;
                    mask[1][(1 + delta[1]) as usize] = true;
                }
            }

            let should_move = mask[0][0] | mask[0][2] | mask[1][0] | mask[1][2];

            if should_move {
                let direction = directions
                    .iter()
                    .find(|&&(ax, delta)| !mask[ax][(1 + delta) as usize]);
                if let Some(&(ax, delta)) = direction {
                    let mut new_pos = elves[elf_num].pos;
                    new_pos[ax] += delta;
                    proposed_moves.push((elf_num, new_pos));
                    *times_proposed.entry(new_pos).or_default() += 1;
                }
                // goto next elf
                i += 1;
            } else {
                elves[elf_num].is_active = false;
                active_elves.swap_remove(i);
            }
        }

        for &(elf_num, new_pos) in proposed_moves.iter() {
            if times_proposed[&new_pos] == 1 {
                elf_positions.remove(&elves[elf_num].pos);
                elf_positions.insert(new_pos, elf_num);
                elves[elf_num].pos = new_pos;
            }
        }

        directions.rotate_left(1);

        turn_counter += 1;

        if turn_limit.map_or(false, |n| n == turn_counter) || proposed_moves.is_empty() {
            return (turn_counter, elf_positions);
        }
    }
}

pub fn part1(input_path: &str) {
    let (_n_turns, elves) = solve(input_path, Some(10));

    let ([xmin, ymin], [xmax, ymax]) = bounding_box(elves.keys().cloned());
    let area = (xmax - xmin + 1) * (ymax - ymin + 1);
    let answer = area as usize - elves.len();
    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let (n_turns, _elves) = solve(input_path, None);
    println!("{}", n_turns);
}

fn parse_elves(input_path: &str) -> Vec<ElfState> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut elves = Vec::new();

    let mut row: i16 = 0;
    let mut col: i16 = 0;
    for &byte in bytes.iter() {
        if byte == b'\n' {
            row += 1;
            col = 0;
        } else {
            if byte == b'#' {
                elves.push(ElfState {
                    is_active: true,
                    pos: [col, row],
                });
            }
            col += 1;
        }
    }
    elves
}

fn bounding_box<const N: usize, I>(mut iterator: I) -> ([i16; N], [i16; N])
where
    I: Iterator<Item = [i16; N]>,
{
    let first = iterator.next().unwrap();
    let mut start = first;
    let mut end = first;

    for pt in iterator {
        for i in 0..N {
            start[i] = min(start[i], pt[i]);
            end[i] = max(end[i], pt[i]);
        }
    }

    (start, end)
}
