use std::collections::HashSet;

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    calc_num_tail_positions(input_path, 2);
}

pub fn part2(input_path: &str) {
    calc_num_tail_positions(input_path, 10);
}

fn calc_num_tail_positions(input_path: &str, rope_length: usize) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);

    let mut rope_state = RopeState::new(rope_length);

    let mut tail_positions = HashSet::new();
    tail_positions.insert(*rope_state.nodes.last().unwrap());

    while reader.has_next() {
        let (direction, count) = read_motion(&mut reader);
        for _ in 0..count {
            rope_state.step(direction);
            tail_positions.insert(*rope_state.nodes.last().unwrap());
        }
    }

    println!("{}", tail_positions.len());
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

struct RopeState {
    nodes: Vec<Coord>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl RopeState {
    fn new(length: usize) -> Self {
        RopeState {
            nodes: vec![Coord { x: 0, y: 0 }; length],
        }
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.nodes[0].y += 1;
            }
            Direction::Down => {
                self.nodes[0].y -= 1;
            }
            Direction::Left => {
                self.nodes[0].x -= 1;
            }
            Direction::Right => {
                self.nodes[0].x += 1;
            }
        }
        for i in 1..self.nodes.len() {
            let dx = self.nodes[i - 1].x - self.nodes[i].x;
            let dy = self.nodes[i - 1].y - self.nodes[i].y;

            if dx.abs() > 1 || dy.abs() > 1 {
                self.nodes[i].x += dx.signum();
                self.nodes[i].y += dy.signum();
            } else {
                break;
            }
        }
    }
}

fn read_motion(reader: &mut Reader) -> (Direction, usize) {
    let direction = match reader.take_byte() {
        b'U' => Direction::Up,
        b'D' => Direction::Down,
        b'L' => Direction::Left,
        b'R' => Direction::Right,
        _ => panic!("invalid direction"),
    };
    reader.skip_lit(b" ");
    let count = reader.read_usize();
    reader.skip_lit(b"\n");
    (direction, count)
}
