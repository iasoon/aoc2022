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

    let num_valves = valve_map.len();
    let &(start_valve_num, _) = &valve_map[b"AA".as_ref()];
    let mut state = State {
        start_pos: start_valve_num,
        current_pos: start_valve_num,

        num_actors: 1,
        current_actor: 0,

        time_available: 30,
        current_time: 0,

        step_log: Vec::new(),

        total_released: 0,
        valve_states: vec![false; valve_map.len()],

        dist_matrix: &dist_matrix,
        flow_rates: &flow_rates,
    };

    state.descend();
    let mut best = state.total_released;

    // descend
    while state.next_state() {
        best = max(best, state.total_released);
    }
    println!("{}", best);
}

pub fn part2(input_path: &str) {
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

    let num_valves = valve_map.len();
    let &(start_valve_num, _) = &valve_map[b"AA".as_ref()];
    let mut state = State {
        start_pos: start_valve_num,
        current_pos: start_valve_num,

        num_actors: 2,
        current_actor: 0,

        time_available: 26,
        current_time: 0,

        step_log: Vec::new(),

        total_released: 0,
        valve_states: vec![false; valve_map.len()],

        dist_matrix: &dist_matrix,
        flow_rates: &flow_rates,
    };

    state.descend();
    let mut best = state.total_released;

    // descend
    while state.next_state() {
        best = max(best, state.total_released);
    }
    println!("{}", best);
}

#[derive(Debug, Copy, Clone)]
enum Action {
    OpenValve(usize),
    NextActor,
}

#[derive(Debug)]
struct Step {
    action: Action,
    prev_pos: usize,
    prev_time: usize,
}

struct State<'a> {
    /// valve number where new actors start
    start_pos: usize,

    step_log: Vec<Step>,

    time_available: usize,

    num_actors: usize,
    current_actor: usize,
    current_pos: usize,
    current_time: usize,

    total_released: usize,

    valve_states: Vec<bool>,

    dist_matrix: &'a VecGrid<usize>,
    flow_rates: &'a [usize],
}

impl<'a> State<'a> {
    fn valve_available(&self, valve_num: usize) -> bool {
        !self.valve_states[valve_num]
            && self.flow_rates[valve_num] > 0
            && self.current_time + self.dist_matrix[(self.current_pos, valve_num)]
                <= self.time_available
    }

    fn apply_action(&mut self, action: Action) {
        let prev_pos = self.current_pos;
        let prev_time = self.current_time;
        match action {
            Action::OpenValve(valve_num) => self.open_valve(valve_num),
            Action::NextActor => self.next_actor(),
        }
        self.step_log.push(Step {
            action,
            prev_pos,
            prev_time,
        });
    }

    fn open_valve(&mut self, valve_num: usize) {
        debug_assert!(!self.valve_states[valve_num]);
        self.current_time += self.dist_matrix[(self.current_pos, valve_num)];
        self.total_released +=
            (self.time_available - self.current_time) * self.flow_rates[valve_num];
        self.valve_states[valve_num] = true;
        self.current_pos = valve_num;
    }

    fn next_actor(&mut self) {
        self.current_actor += 1;
        debug_assert!(self.current_actor < self.num_actors);
        self.current_pos = self.start_pos;
        self.current_time = 0;
    }

    fn undo_step(&mut self, step: &Step) {
        match step.action {
            Action::OpenValve(valve_num) => {
                self.total_released -=
                    (self.time_available - self.current_time) * self.flow_rates[valve_num];
                self.valve_states[valve_num] = false;
            }
            Action::NextActor => {
                self.current_actor -= 1;
            }
        }
        self.current_time = step.prev_time;
        self.current_pos = step.prev_pos;
    }

    fn num_valves(&self) -> usize {
        self.flow_rates.len()
    }

    fn descend(&mut self) {
        while let Some(action) = self.first_available_action(0) {
            self.apply_action(action);
        }
    }

    fn first_available_action(&self, first_allowed_valve: usize) -> Option<Action> {
        let next_available_valve =
            (first_allowed_valve..self.num_valves()).find(|&v| self.valve_available(v));
        if let Some(valve_num) = next_available_valve {
            Some(Action::OpenValve(valve_num))
        } else if self.current_actor + 1 < self.num_actors {
            Some(Action::NextActor)
        } else {
            None
        }
    }

    fn next_action(&self, action: Action) -> Option<Action> {
        match action {
            Action::OpenValve(prev_valve) => self.first_available_action(prev_valve + 1),
            Action::NextActor => None,
        }
    }

    // returns: success state
    fn next_state(&mut self) -> bool {
        while let Some(step) = self.step_log.pop() {
            self.undo_step(&step);
            if let Some(next_action) = self.next_action(step.action) {
                self.apply_action(next_action);
                self.descend();
                return true;
            }
        }
        // all possible states exhausted
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
