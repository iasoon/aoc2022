use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
};

use crate::utils::{Reader, VecGrid};

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut valve_map = HashMap::new();
    for valve in read_valves(&mut reader).into_iter() {
        let valve_num = valve_map.len();
        valve_map.insert(valve.name, (valve_num, valve));
    }
    let mut flow_rates = vec![0; valve_map.len()];
    let mut dist_matrix = VecGrid::full(valve_map.len(), valve_map.len(), 0);
    for (name, &(num, ref valve)) in valve_map.iter() {
        flow_rates[num] = valve.flow_rate;
        fill_valve_distances(&mut dist_matrix, &valve_map, name);
    }

    // for i in 0..dist_matrix.width() {
    //     println!("VALVE {}, flow rate={}", i, flow_rates[i]);
    //     for j in 0..dist_matrix.height() {
    //         println!("dist to {} = {}", j, dist_matrix[(i, j)]);
    //     }
    //     println!();
    // }
    let num_valves = valve_map.len();
    let &(start_valve_num, _) = &valve_map[b"AA".as_ref()];
    let mut state = State {
        pos: start_valve_num,
        minutes_remaining: 30,
        total_released: 0,
        release_rate: 0,
        valve_states: vec![false; valve_map.len()],

        dist_matrix: &dist_matrix,
        flow_rates: &flow_rates,

        history: Vec::new(),
    };

    state.descend();
    let mut best = state.projected_release();

    // descend
    while state.next_state() {
        // println!("hist {:?}, pos {}", state.history, state.pos);
        // println!("remaining {}", state.minutes_remaining);
        // println!("release: {}", state.projected_release());
        best = max(best, state.projected_release());
        state.next_state();
    }
    println!("best: {}", best);
}

struct State<'a> {
    pos: usize,
    minutes_remaining: usize,
    total_released: usize,
    release_rate: usize,
    valve_states: Vec<bool>,

    dist_matrix: &'a VecGrid<usize>,
    flow_rates: &'a [usize],
    // in which order the valves were opened
    history: Vec<usize>,
}

impl<'a> State<'a> {
    fn valve_available(&self, valve_num: usize) -> bool {
        !self.valve_states[valve_num]
            && self.flow_rates[valve_num] > 0
            && self.dist_matrix[(self.pos, valve_num)] <= self.minutes_remaining
    }

    fn open_valve(&mut self, valve_num: usize) {
        debug_assert!(!self.valve_states[valve_num]);
        self.history.push(self.pos);
        let minutes_elapsed = self.dist_matrix[(self.pos, valve_num)];
        self.minutes_remaining -= minutes_elapsed;
        self.total_released += minutes_elapsed * self.release_rate;
        self.pos = valve_num;
        self.valve_states[valve_num] = true;
        self.release_rate += self.flow_rates[valve_num];
    }

    fn backtrack(&mut self) {
        // self.pos is the valve that was opened last
        let prev_pos = self.history.pop().unwrap();
        self.release_rate -= self.flow_rates[self.pos];
        self.valve_states[self.pos] = false;
        let minutes_elapsed = self.dist_matrix[(prev_pos, self.pos)];
        self.total_released -= minutes_elapsed * self.release_rate;
        self.minutes_remaining += minutes_elapsed;
        self.pos = prev_pos;
    }

    fn projected_release(&self) -> usize {
        self.total_released + self.minutes_remaining * self.release_rate
    }

    fn num_valves(&self) -> usize {
        self.flow_rates.len()
    }

    fn descend(&mut self) {
        while let Some(v) = (0..self.num_valves()).find(|&v| self.valve_available(v)) {
            self.open_valve(v);
        }
    }

    fn next_state(&mut self) -> bool {
        while !self.history.is_empty() {
            let prev = self.pos;
            self.backtrack();
            if let Some(v) = (prev + 1..self.num_valves()).find(|&v| self.valve_available(v)) {
                self.open_valve(v);
                self.descend();
                return true;
            }
        }
        false
    }
}

type ValveMap<'a> = HashMap<&'a [u8], (usize, Valve<'a>)>;

fn fill_valve_distances(dist_matrix: &mut VecGrid<usize>, valve_map: &ValveMap, start: &[u8]) {
    let mut visited = vec![false; valve_map.len()];
    let mut queue = VecDeque::new();
    let &(origin_num, _) = &valve_map[start];
    queue.push_back((0, start));
    while let Some((dist, name)) = queue.pop_front() {
        match valve_map.get(name) {
            Some(&(num, ref valve)) if !visited[num] => {
                visited[num] = true;
                dist_matrix[(origin_num, num)] = dist + 1; // +1 for opening the valve
                for neighbour in valve.neighbours.iter() {
                    queue.push_back((dist + 1, neighbour));
                }
            }
            _ => continue,
        };
    }
}

pub fn part2(input_path: &str) {
    todo!()
}

struct Valve<'a> {
    name: &'a [u8],
    flow_rate: usize,
    neighbours: Vec<&'a [u8]>,
}

fn read_valves<'a>(reader: &mut Reader<'a>) -> Vec<Valve<'a>> {
    let mut valves = Vec::new();
    while reader.has_next() {
        let valve = read_valve(reader);
        valves.push(valve);
    }
    return valves;
}

fn read_valve<'a>(reader: &mut Reader<'a>) -> Valve<'a> {
    let mut neighbours = Vec::new();
    reader.skip_lit(b"Valve ");
    let name = reader.take_while(|c| c != b' ');
    reader.skip_lit(b" has flow rate=");
    let flow_rate = reader.read_usize();
    reader.skip_lit(b"; tunnel");
    if reader.peek() == b's' {
        reader.skip_lit(b"s lead to valves ");
    } else {
        reader.skip_lit(b" leads to valve ");
    }

    loop {
        if reader.peek() == b'\n' {
            reader.skip_lit(b"\n");
            break;
        } else if reader.peek() == b',' {
            reader.skip_lit(b", ");
        }
        let neighbour = reader.take_while(|c| c != b',' && c != b'\n');
        neighbours.push(neighbour)
    }
    Valve {
        name,
        flow_rate,
        neighbours,
    }
}
