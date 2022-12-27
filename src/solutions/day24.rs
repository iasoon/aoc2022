use std::collections::HashSet;

use crate::utils::{FnvHash, Reader, VecGrid};

pub fn part1(input_path: &str) {
    solve(input_path, false);
}

pub fn part2(input_path: &str) {
    solve(input_path, true);
}

fn solve(input_path: &str, part2: bool) {
    let bytes = std::fs::read(input_path).unwrap();
    let (width, height) = get_dimensions(&bytes);
    let mut times_blocked: VecGrid<usize> = VecGrid::full(width, height, 0);
    let mut blizzards = Vec::new();

    let mut reader = Reader::from_bytes(&bytes);
    for y in 0..height {
        for x in 0..width {
            let byte = reader.take_byte();
            if byte != b'.' {
                times_blocked[(x, y)] += 1;
            }
            if let Some(direction) = parse_direction(byte) {
                blizzards.push(Blizzard {
                    pos: (x, y),
                    direction,
                })
            }
        }
        reader.skip_lit(b"\n");
    }

    let start = (1, 0);
    let exit = (width - 2, height - 1);

    let targets = if part2 {
        vec![exit, start, exit]
    } else {
        vec![exit]
    };

    debug_assert!(times_blocked[start] == 0);
    debug_assert!(times_blocked[exit] == 0);
    let mut positions = HashSet::with_hasher(FnvHash);
    positions.insert(start);

    let mut turn_num = 0;
    let mut target_num = 0;
    loop {
        if positions.contains(&targets[target_num]) {
            if target_num == targets.len() - 1 {
                println!("{}", turn_num);
                return;
            } else {
                // move to next target
                positions.clear();
                positions.insert(targets[target_num]);
                target_num += 1;
            }
        }
        // step blizzards
        for blizzard in blizzards.iter_mut() {
            let (x, y) = blizzard.pos;
            times_blocked[blizzard.pos] -= 1;
            let (mut x_next, mut y_next) = match blizzard.direction {
                Direction::North => (x, y - 1),
                Direction::South => (x, y + 1),
                Direction::East => (x + 1, y),
                Direction::West => (x - 1, y),
            };

            if x_next == 0 {
                x_next = width - 2;
            }
            if x_next == width - 1 {
                x_next = 1;
            }
            if y_next == 0 {
                y_next = height - 2;
            }
            if y_next == height - 1 {
                y_next = 1;
            }
            blizzard.pos = (x_next, y_next);
            times_blocked[blizzard.pos] += 1;
        }

        // create new position set
        let mut next_positions = HashSet::with_hasher(FnvHash);
        for (x, y) in positions.into_iter() {
            let next_xy = [
                Some((x, y)),
                Some((x + 1, y)),
                Some((x - 1, y)),
                (y > 0).then(|| (x, y - 1)),
                (y < height - 1).then(|| (x, y + 1)),
            ]
            .into_iter()
            .flatten();

            for pos in next_xy {
                if times_blocked[pos] == 0 {
                    next_positions.insert(pos);
                }
            }
        }
        positions = next_positions;
        turn_num += 1;
    }
}

struct Blizzard {
    pos: (usize, usize),
    direction: Direction,
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_direction(byte: u8) -> Option<Direction> {
    match byte {
        b'>' => Some(Direction::East),
        b'<' => Some(Direction::West),
        b'^' => Some(Direction::North),
        b'v' => Some(Direction::South),
        _ => None,
    }
}

fn get_dimensions(bytes: &[u8]) -> (usize, usize) {
    let mut reader = Reader::from_bytes(bytes);
    let line = reader.take_while(|b| b != b'\n');
    let width = line.len();
    let line_length = width + 1; // add newline
    debug_assert!(bytes.len() % line_length == 0);
    (width, bytes.len() / line_length)
}
