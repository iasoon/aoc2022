use std::{cmp::max, collections::HashSet, ops::Range};

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let blueprints = parse_blueprints(input_path);
    let answer: usize = blueprints
        .iter()
        .map(|bp| bp.id * bp.calc_max_opened_geodes(24))
        .sum();
    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let blueprints = parse_blueprints(input_path);
    let answer: usize = blueprints
        .iter()
        .take(3)
        .map(|bp| bp.calc_max_opened_geodes(32))
        .product();
    println!("{}", answer);
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    minutes_elapsed: usize,
    resources: [usize; 4],
    robots: [usize; 4],
}

struct Blueprint {
    id: usize,
    cost_matrix: [[usize; 4]; 4],
}

const RESOURCES: Range<usize> = 0..4;

fn minutes_to_construct(
    resource_num: usize,
    state: &State,
    blueprint: &Blueprint,
) -> Option<usize> {
    let cost = blueprint.cost_matrix[resource_num];
    let mut minutes = 0;
    for i in RESOURCES {
        let needed = cost[i].saturating_sub(state.resources[i]);
        if needed > 0 {
            if state.robots[i] == 0 {
                // we can never get enough of this resource
                return None;
            }
            let m = div_ceil(needed, state.robots[i]);
            minutes = max(minutes, m);
        }
    }
    // add 1 for actually constructing the robot
    Some(1 + minutes)
}

fn div_ceil(dividend: usize, divisor: usize) -> usize {
    (dividend / divisor) + (dividend % divisor > 0) as usize
}

impl Blueprint {
    fn calc_max_opened_geodes(&self, n_minutes: usize) -> usize {
        let initial_state = State {
            minutes_elapsed: 0,
            resources: [0; 4],
            robots: [1, 0, 0, 0],
        };

        let mut max_useful_robots = [0; 4];
        for robot_num in RESOURCES {
            RESOURCES.for_each(|resource_num| {
                max_useful_robots[resource_num] = max(
                    max_useful_robots[resource_num],
                    self.cost_matrix[robot_num][resource_num],
                );
            });
        }
        max_useful_robots[GEODE] = usize::max_value();

        let mut best = 0;

        let mut stack = StateStack::new();
        stack.push(initial_state);
        while let Some(state) = stack.pop() {
            let mut any_added = false;
            for resource_num in RESOURCES {
                if state.robots[resource_num] >= max_useful_robots[resource_num] {
                    continue;
                }
                if let Some(to_construct) = minutes_to_construct(resource_num, &state, self) {
                    let minutes_elapsed = state.minutes_elapsed + to_construct;
                    if minutes_elapsed > n_minutes {
                        continue;
                    }
                    let mut resources = state.resources;
                    for i in RESOURCES {
                        resources[i] += to_construct * state.robots[i];
                        resources[i] -= self.cost_matrix[resource_num][i];
                    }
                    let mut robots = state.robots;
                    robots[resource_num] += 1;
                    let next_state = State {
                        minutes_elapsed,
                        resources,
                        robots,
                    };
                    stack.push(next_state);
                    any_added = true;
                }
            }
            if !any_added {
                let remaining = n_minutes - state.minutes_elapsed;
                let score = state.resources[GEODE] + remaining * state.robots[GEODE];
                best = max(best, score);
            }
        }

        best
    }
}

struct StateStack {
    stack: Vec<State>,
    seen: HashSet<State>,
}

impl StateStack {
    fn new() -> Self {
        StateStack {
            stack: Vec::new(),
            seen: HashSet::new(),
        }
    }
    fn push(&mut self, state: State) {
        if !self.seen.contains(&state) {
            self.seen.insert(state.clone());
            self.stack.push(state);
        }
    }

    fn pop(&mut self) -> Option<State> {
        self.stack.pop()
    }
}

fn parse_blueprints(input_path: &str) -> Vec<Blueprint> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut blueprints = Vec::new();
    while reader.has_next() {
        let blueprint = read_blueprint(&mut reader);
        blueprints.push(blueprint);
    }
    blueprints
}

fn read_blueprint(reader: &mut Reader) -> Blueprint {
    let mut cost_matrix = [[0; 4]; 4];
    reader.skip_lit(b"Blueprint ");
    let id = reader.read_usize();
    reader.skip_lit(b": Each ore robot costs ");
    cost_matrix[ORE][ORE] = reader.read_usize();
    reader.skip_lit(b" ore. Each clay robot costs ");
    cost_matrix[CLAY][ORE] = reader.read_usize();
    reader.skip_lit(b" ore. Each obsidian robot costs ");
    cost_matrix[OBSIDIAN][ORE] = reader.read_usize();
    reader.skip_lit(b" ore and ");
    cost_matrix[OBSIDIAN][CLAY] = reader.read_usize();
    reader.skip_lit(b" clay. Each geode robot costs ");
    cost_matrix[GEODE][ORE] = reader.read_usize();
    reader.skip_lit(b" ore and ");
    cost_matrix[GEODE][OBSIDIAN] = reader.read_usize();
    reader.skip_lit(b" obsidian.\n");
    Blueprint { id, cost_matrix }
}
