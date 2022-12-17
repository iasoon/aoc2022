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

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let directions = if bytes.last() == Some(&b'\n') {
        &bytes[..bytes.len() - 1]
    } else {
        &bytes
    };
    let mut direction_counter = 0;

    let mut row_stack: Vec<Vec<bool>> = Vec::new();
    for rock_num in 0..2022 {
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
        }
    }

    println!("{}", row_stack.len());
}

pub fn part2(input_path: &str) {
    todo!()
}
