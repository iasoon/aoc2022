use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use crate::utils::{add_vecs, FnvHash, FnvHashMap, FnvHashSet};

pub fn solve(input_path: &str, turn_limit: Option<usize>) -> (usize, FnvHashSet<[isize; 2]>) {
    let mut elves = parse_elves(input_path);

    let mut directions: [[isize; 2]; 4] = [
        [0, -1], // North
        [0, 1],  // Sounth
        [-1, 0], // West
        [1, 0],  // East
    ];
    let mut times_proposed: FnvHashMap<[isize; 2], usize> = HashMap::with_hasher(FnvHash);
    let mut proposed_moves = Vec::new();

    let mut turn_counter = 0;

    loop {
        proposed_moves.clear();
        times_proposed.clear();
        for &elf_pos in elves.iter() {
            let mut mask = [[false; 3]; 2];

            for dx in [-1isize, 0, 1] {
                for dy in [-1isize, 0, 1] {
                    let p = add_vecs(&elf_pos, &[dx, dy]);
                    let has_elf = elves.contains(&p);
                    mask[0][(1 + dx) as usize] |= has_elf;
                    mask[1][(1 + dy) as usize] |= has_elf;
                }
            }

            if !(mask[0][0] | mask[0][2] | mask[1][0] | mask[1][2]) {
                continue;
            }

            let direction = directions.iter().find(|direction| {
                let i = direction[1].unsigned_abs();
                !mask[i][(1 + direction[i]) as usize]
            });
            if let Some(d) = direction {
                let new_pos = add_vecs(&elf_pos, d);
                proposed_moves.push((elf_pos, new_pos));
                *times_proposed.entry(new_pos).or_default() += 1;
            }
        }
        for (old_pos, new_pos) in proposed_moves.iter().cloned() {
            if times_proposed[&new_pos] == 1 {
                elves.remove(&old_pos);
                elves.insert(new_pos);
            }
        }

        directions.rotate_left(1);

        turn_counter += 1;

        if turn_limit.map_or(false, |n| n == turn_counter) || proposed_moves.is_empty() {
            return (turn_counter, elves);
        }
    }
}

pub fn part1(input_path: &str) {
    let (_n_turns, elves) = solve(input_path, Some(10));

    let ([xmin, ymin], [xmax, ymax]) = bounding_box(elves.iter().cloned());
    let area = (xmax - xmin + 1) * (ymax - ymin + 1);
    let answer = area as usize - elves.len();
    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let (n_turns, _elves) = solve(input_path, None);
    println!("{}", n_turns);
}

fn parse_elves(input_path: &str) -> FnvHashSet<[isize; 2]> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut elves = HashSet::with_hasher(FnvHash);

    let mut row: isize = 0;
    let mut col: isize = 0;
    for &byte in bytes.iter() {
        if byte == b'\n' {
            row += 1;
            col = 0;
        } else {
            if byte == b'#' {
                elves.insert([col, row]);
            }
            col += 1;
        }
    }
    elves
}

fn bounding_box<const N: usize, I>(mut iterator: I) -> ([isize; N], [isize; N])
where
    I: Iterator<Item = [isize; N]>,
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
