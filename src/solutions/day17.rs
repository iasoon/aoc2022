use std::{collections::HashMap, usize};

struct Shape {
    height: usize,
    width: usize,
    points: &'static [(usize, usize)],
}

static SHAPES: &[Shape] = &[
    Shape {
        height: 1,
        width: 4,
        points: &[(0, 0), (1, 0), (2, 0), (3, 0)],
    },
    Shape {
        height: 3,
        width: 3,
        points: &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    },
    Shape {
        height: 3,
        width: 3,
        points: &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    },
    Shape {
        height: 4,
        width: 1,
        points: &[(0, 0), (0, 1), (0, 2), (0, 3)],
    },
    Shape {
        height: 2,
        width: 2,
        points: &[(0, 0), (0, 1), (1, 0), (1, 1)],
    },
];

pub fn part1(input_path: &str) {
    solve(input_path, 2022);
}

pub fn part2(input_path: &str) {
    solve(input_path, 1000000000000);
}

struct RockFormation {
    shape: &'static Shape,
    x_offset: usize,
    y_offset: usize,
}

impl RockFormation {
    fn points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.shape
            .points
            .iter()
            .map(|(x, y)| (self.x_offset + x, self.y_offset + y))
    }
    fn can_move_left(&self, row_stack: &[Vec<bool>]) -> bool {
        if self.x_offset == 0 {
            return false;
        }

        self.points()
            .all(|(x, y)| y >= row_stack.len() || !row_stack[y][x - 1])
    }

    fn can_move_right(&self, row_stack: &[Vec<bool>]) -> bool {
        if self.x_offset == 7 - self.shape.width {
            return false;
        }

        self.points()
            .all(|(x, y)| y >= row_stack.len() || !row_stack[y][x + 1])
    }

    fn can_move_down(&self, row_stack: &[Vec<bool>]) -> bool {
        if self.y_offset == 0 {
            return false;
        }

        self.points()
            .all(|(x, y)| y > row_stack.len() || !row_stack[y - 1][x])
    }
}

fn solve(input_path: &str, n_rocks: usize) {
    let bytes = std::fs::read(input_path).unwrap();
    let directions = if bytes.last() == Some(&b'\n') {
        &bytes[..bytes.len() - 1]
    } else {
        &bytes
    };
    let answer = calc_stack_height(directions, n_rocks);
    println!("{}", answer);
}

fn calc_stack_height(directions: &[u8], n_rocks: usize) -> usize {
    let mut direction_counter = 0;

    let mut height_per_col = vec![0; 7];

    let mut row_stack: Vec<Vec<bool>> = Vec::new();
    let mut height_log = Vec::new();
    let mut pattern_index = HashMap::new();

    let mut rock_num = 0;
    while rock_num < n_rocks {
        let mut rock = RockFormation {
            shape: &SHAPES[rock_num % SHAPES.len()],
            x_offset: 2,
            y_offset: row_stack.len() + 3,
        };
        loop {
            let b = directions[direction_counter % directions.len()];
            direction_counter += 1;
            match b {
                b'<' => {
                    if rock.can_move_left(&row_stack) {
                        rock.x_offset -= 1;
                    }
                }
                b'>' => {
                    if rock.can_move_right(&row_stack) {
                        rock.x_offset += 1;
                    }
                }
                _ => panic!("bad direction"),
            }

            if rock.can_move_down(&row_stack) {
                rock.y_offset -= 1;
            } else {
                break;
            }
        }
        for _ in row_stack.len()..(rock.y_offset + rock.shape.height) {
            row_stack.push(vec![false; 7]);
        }

        for (x, y) in rock.points() {
            row_stack[y][x] = true;
            if y > height_per_col[x] {
                height_per_col[x] = y;
            }
        }

        let baseline = height_per_col.iter().min().unwrap();
        let rel_baseline: Vec<usize> = height_per_col.iter().map(|h| h - baseline).collect();
        let key = (
            rel_baseline,
            rock_num % SHAPES.len(),
            direction_counter % directions.len(),
        );

        if let Some(&cycle_start) = pattern_index.get(&key) {
            // Found a cycle!
            // We can now skip along the cycle until we approach the desired
            // end state.

            let current_height = row_stack.len();

            let start_height = height_log[cycle_start];
            let cycle_len = rock_num - cycle_start;
            let cycle_height_diff = current_height - start_height;

            // -1 because rock_num starts at 0
            let rocks_remaining = n_rocks - rock_num - 1;
            let full_cycles_remaining = rocks_remaining / cycle_len;

            let remaining_cycles_height = cycle_height_diff * full_cycles_remaining;

            // final position modulo cycle length, so that it ends up at a position
            // that we already know the height for
            let end_state = cycle_start + rocks_remaining % cycle_len;
            let remainder_height = height_log[end_state] - start_height;

            return current_height + remaining_cycles_height + remainder_height;
        }

        pattern_index.insert(key, rock_num);
        height_log.push(row_stack.len());
        rock_num += 1;
    }

    // we did not find a cycle
    row_stack.len()
}
