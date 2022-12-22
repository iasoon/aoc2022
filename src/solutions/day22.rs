use std::collections::HashMap;

use crate::utils::{add_vecs, Reader};

type Map = HashMap<[isize; 2], u8>;

fn find_edge(map: &Map, mut pos: [isize; 2], direction: [isize; 2]) -> [isize; 2] {
    loop {
        let next_pos = add_vecs(&pos, &direction);
        if map.get(&next_pos).is_some() {
            pos = next_pos;
        } else {
            return pos;
        }
    }
}

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let (map, steps) = parse_map(&bytes);
    let mut pos = *map.keys().min().unwrap();
    let mut direction = [0, 1];
    for step in steps.into_iter() {
        match step {
            Step::Forward(count) => {
                for _ in 0..count {
                    let next_pos = add_vecs(&pos, &direction);
                    match map.get(&next_pos) {
                        Some(b'.') => {
                            // we can move here
                            pos = next_pos;
                        }
                        Some(_) => break,
                        None => {
                            let edge = find_edge(&map, pos, invert(direction));
                            if map[&edge] == b'.' {
                                pos = edge;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            Step::TurnLeft => {
                direction = turn_left(direction);
            }
            Step::TurnRight => {
                direction = turn_right(direction);
            }
        }
    }

    let row_num = 1 + pos[0];
    let col_num = 1 + pos[1];
    let facing_num = match direction {
        [0, 1] => 0,
        [1, 0] => 1,
        [0, -1] => 2,
        [-1, 0] => 3,
        _ => panic!("bad direction"),
    };

    let answer = 1000 * row_num + 4 * col_num + facing_num;
    println!("{}", answer);
}

fn parse_map(bytes: &[u8]) -> (HashMap<[isize; 2], u8>, Vec<Step>) {
    let mut reader = Reader::from_bytes(bytes);
    let mut row: isize = 0;
    let mut map = HashMap::new();
    while reader.peek() != b'\n' {
        let blank_space = reader.take_while(|b| b == b' ');
        let mut col = blank_space.len() as isize;
        while reader.peek() != b'\n' {
            map.insert([row, col], reader.take_byte());
            col += 1;
        }
        reader.skip_lit(b"\n");
        row += 1;
    }
    reader.skip_lit(b"\n");
    let mut steps = Vec::new();
    while reader.has_next() {
        let step = match reader.peek() {
            b'0'..=b'9' => {
                let n_steps = reader.read_isize();
                Step::Forward(n_steps)
            }
            b'L' => {
                reader.skip_lit(b"L");
                Step::TurnLeft
            }
            b'R' => {
                reader.skip_lit(b"R");
                Step::TurnRight
            }
            _ => break,
        };
        steps.push(step);
    }
    assert!(reader.peek() == b'\n');
    (map, steps)
}

#[derive(Debug)]
enum Step {
    Forward(isize),
    TurnLeft,
    TurnRight,
}

fn turn_right(direction: [isize; 2]) -> [isize; 2] {
    [direction[1], -direction[0]]
}

fn turn_left(direction: [isize; 2]) -> [isize; 2] {
    [-direction[1], direction[0]]
}

fn invert(direction: [isize; 2]) -> [isize; 2] {
    [-direction[0], -direction[1]]
}

pub fn part2(input_path: &str) {
    todo!()
}
