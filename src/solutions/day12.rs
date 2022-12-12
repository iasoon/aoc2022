use std::collections::VecDeque;

pub fn part1(input_path: &str) {
    let mut bytes = std::fs::read(input_path).unwrap();
    let info = read_info(&bytes);
    bytes[info.start_pos] = b'a';
    bytes[info.end_pos] = b'z';
    let mut state = State {
        bytes: &bytes,
        queue: VecDeque::new(),
        backlinks: vec![None; bytes.len()],
        line_length: info.line_length,
    };
    state.queue.push_back((info.start_pos, info.start_pos));
    state.find_path(info.end_pos);
    let steps = state.count_steps(info.end_pos);

    println!("steps: {}", steps);
}

pub fn part2(input_path: &str) {
    let mut bytes = std::fs::read(input_path).unwrap();
    let info = read_info(&bytes);
    bytes[info.start_pos] = b'a';
    bytes[info.end_pos] = b'z';
    let mut state = State {
        bytes: &bytes,
        queue: VecDeque::new(),
        backlinks: vec![None; bytes.len()],
        line_length: info.line_length,
    };
    for (i, &c) in bytes.iter().enumerate() {
        if c == b'a' {
            state.queue.push_back((i, i));
        }
    }
    state.find_path(info.end_pos);
    let steps = state.count_steps(info.end_pos);

    println!("steps: {}", steps);
}

struct State<'a> {
    line_length: usize,
    bytes: &'a [u8],
    // (prev, pos)
    queue: VecDeque<(usize, usize)>,
    // pos -> prev
    backlinks: Vec<Option<usize>>,
}

impl<'a> State<'a> {
    fn visit_neighbours(&mut self, pos: usize) {
        if pos % self.line_length > 0 {
            self.visit_neighbour(pos, pos - 1);
        }
        if pos % self.line_length < self.line_length - 1 {
            self.visit_neighbour(pos, pos + 1);
        }
        if pos > self.line_length {
            self.visit_neighbour(pos, pos - self.line_length);
        }
        if pos < self.bytes.len() - self.line_length {
            self.visit_neighbour(pos, pos + self.line_length);
        }
    }

    fn visit_neighbour(&mut self, from: usize, pos: usize) {
        if self.backlinks[pos].is_none() && self.bytes[pos] <= self.bytes[from] + 1 {
            self.queue.push_back((from, pos))
        }
    }

    fn find_path(&mut self, end_pos: usize) {
        while let Some((prev, pos)) = self.queue.pop_front() {
            if self.backlinks[pos].is_none() {
                self.backlinks[pos] = Some(prev);
                if pos == end_pos {
                    return;
                } else {
                    self.visit_neighbours(pos);
                }
            }
        }
    }

    fn count_steps(&mut self, target: usize) -> usize {
        let mut steps = 0;
        let mut pos = target;
        loop {
            let prev = self.backlinks[pos].unwrap();
            if prev == pos {
                return steps;
            }
            pos = prev;
            steps += 1;
        }
    }
}

#[derive(Debug)]
struct Info {
    line_length: usize,
    start_pos: usize,
    end_pos: usize,
}

fn read_info(bytes: &[u8]) -> Info {
    let mut line_length = None;
    let mut start_pos = None;
    let mut end_pos = None;

    for (i, c) in bytes.iter().enumerate() {
        match c {
            b'S' => start_pos = Some(i),
            b'E' => end_pos = Some(i),
            b'\n' => {
                if line_length.is_none() {
                    line_length = Some(i + 1);
                }
            }
            _ => (),
        }
    }

    Info {
        line_length: line_length.unwrap(),
        start_pos: start_pos.unwrap(),
        end_pos: end_pos.unwrap(),
    }
}
