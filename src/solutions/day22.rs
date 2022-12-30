use std::collections::HashMap;

use crate::utils::{add_vecs, Reader};

type Map = HashMap<[isize; 2], u8>;

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let (map, steps) = parse_input(&bytes);
    let answer = follow_path(&map, &steps, |pos, direction| {
        let edge = find_edge(&map, pos, invert_vec(direction));
        (edge, direction)
    });
    println!("{}", answer);
}

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

fn follow_path<F>(map: &Map, steps: &[Step], wrap_fn: F) -> isize
where
    F: Fn([isize; 2], [isize; 2]) -> ([isize; 2], [isize; 2]),
{
    let mut pos = *map.keys().min_by_key(|&[x, y]| [y, x]).unwrap();
    let mut direction = [1, 0];
    for step in steps {
        match step {
            &Step::Forward(count) => {
                for _ in 0..count {
                    let next_pos = add_vecs(&pos, &direction);
                    match map.get(&next_pos) {
                        Some(b'.') => {
                            // we can move here
                            pos = next_pos;
                        }
                        Some(_) => break,
                        None => {
                            let (next_pos, next_direction) = wrap_fn(pos, direction);
                            if map[&next_pos] == b'.' {
                                pos = next_pos;
                                direction = next_direction;
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

    calc_answer(pos, direction)
}

fn calc_answer(pos: [isize; 2], direction: [isize; 2]) -> isize {
    let row_num = 1 + pos[1];
    let col_num = 1 + pos[0];
    let facing_num = match direction {
        [1, 0] => 0,
        [0, 1] => 1,
        [-1, 0] => 2,
        [0, -1] => 3,
        _ => panic!("bad direction"),
    };

    1000 * row_num + 4 * col_num + facing_num
}

fn parse_input(bytes: &[u8]) -> (Map, Vec<Step>) {
    let mut reader = Reader::from_bytes(bytes);
    let mut y: isize = 0;
    let mut map = HashMap::new();
    while reader.peek() != b'\n' {
        let blank_space = reader.take_while(|b| b == b' ');
        let mut x = blank_space.len() as isize;
        while reader.peek() != b'\n' {
            map.insert([x, y], reader.take_byte());
            x += 1;
        }
        reader.skip_lit(b"\n");
        y += 1;
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
    [-direction[1], direction[0]]
}

fn turn_left(direction: [isize; 2]) -> [isize; 2] {
    [direction[1], -direction[0]]
}

const CUBE_WIDTH: isize = 50;

pub fn part2(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let (map, steps) = parse_input(&bytes);

    // pick an arbitrary starting point
    let pos = *map.keys().min().unwrap();

    // elect this pane as the front of the cube.
    let normal = [0, 0, -1];
    let rot_mat = EYE;

    // Find all cube sides using a DFS, keeping track of their rotations.
    let mut stack = Vec::new();
    stack.push((pos, rot_mat));

    // rotation matrices for the four cardinal directions
    let rot_down = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
    let rot_up = [[1, 0, 0], [0, 0, 1], [0, -1, 0]];
    let rot_left = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
    let rot_right = [[0, 0, -1], [0, 1, 0], [1, 0, 0]];

    let mut pane_rotations: HashMap<[isize; 2], [[isize; 3]; 3]> = HashMap::new();
    while let Some((pos, rot_mat)) = stack.pop() {
        if let Some(mat) = pane_rotations.get(&pos) {
            // sanity check
            debug_assert!(mat == &rot_mat);
        } else if map.contains_key(&pos) {
            pane_rotations.insert(pos, rot_mat);
            let ngbrs = [
                ([0, CUBE_WIDTH], rot_down),
                ([0, -CUBE_WIDTH], rot_up),
                ([CUBE_WIDTH, 0], rot_right),
                ([-CUBE_WIDTH, 0], rot_left),
            ];
            for (dv, rot) in ngbrs {
                stack.push((add_vecs(&pos, &dv), matmul(&rot_mat, &rot)));
            }
        }
    }

    // map outwards pointing normal vector to a pane for easy lookups
    let normal_to_pane: HashMap<_, _> = pane_rotations
        .iter()
        .map(|(topleft, rot)| (vecmul(rot, &normal), topleft))
        .collect();

    let answer = follow_path(&map, &steps, |pos, direction| {
        // find the pane we are in, and the rotation associated with it.
        let origin_pane = [
            pos[0] / CUBE_WIDTH * CUBE_WIDTH,
            pos[1] / CUBE_WIDTH * CUBE_WIDTH,
        ];
        let origin_rot = &pane_rotations[&origin_pane];

        // rotate the local direction vector to a 3d vector in our cube.
        let local_direction_3d = [direction[0], direction[1], 0];
        let target_normal = vecmul(origin_rot, &local_direction_3d);
        // the pane whose outward normal vector aligns with the 3d direction vector we 'drop' in,
        // is the one we should arrive at.
        let target_pane = normal_to_pane[&target_normal];

        // When we drop off the pane, we will always make a 90 degree angle towards the center of
        // the cube in 3d. This means we will always arrive in the direction of the inwards
        // pointing normal of the origin pane.
        let origin_inner_normal = vecmul(origin_rot, &[0, 0, 1]);

        // Rotate the origins inner normal to the reference point of the target pane - this should
        // always result in a vector with a zero z-component.
        // This vector is the direction we arrive in at the target pane.
        let target_rot = &pane_rotations[target_pane];
        let inv_target_rot = transpose(target_rot);
        let target_direction_3d = vecmul(&inv_target_rot, &origin_inner_normal);
        let target_direction = [target_direction_3d[0], target_direction_3d[1]];

        // Now we need to find the position on the target pane we will arrive at.
        // Since we know the panes have to align, there will be the same amount of distance on your
        // left (and on your right) before and after wrapping around the edge.
        // We first translate from (pos, direction) to dist_on_the_left, then back from (direction,
        // dist_on_the_left) to the position we will arrive in.
        // this part is a bit nasty; because the y-axis is flipped we cannot use a general formula
        // here.
        let rel_pos = add_vecs(&pos, &invert_vec(origin_pane));
        let dist_on_the_left = match direction {
            [1, 0] => rel_pos[1],
            [-1, 0] => CUBE_WIDTH - 1 - rel_pos[1],
            [0, 1] => CUBE_WIDTH - 1 - rel_pos[0],
            [0, -1] => rel_pos[0],
            _ => unreachable!(),
        };

        let rel_target_pos = match target_direction {
            [1, 0] => [0, dist_on_the_left],
            [-1, 0] => [CUBE_WIDTH - 1, CUBE_WIDTH - 1 - dist_on_the_left],
            [0, 1] => [CUBE_WIDTH - 1 - dist_on_the_left, 0],
            [0, -1] => [dist_on_the_left, CUBE_WIDTH - 1],
            _ => unreachable!(),
        };
        let target_pos = add_vecs(&rel_target_pos, target_pane);

        (target_pos, target_direction)
    });
    println!("{}", answer);
}

const EYE: [[isize; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

fn matmul<const N: usize>(a: &[[isize; N]; N], b: &[[isize; N]; N]) -> [[isize; N]; N] {
    let mut res = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

fn vecmul<const N: usize>(a: &[[isize; N]; N], vec: &[isize; N]) -> [isize; N] {
    let mut res = [0; N];
    for i in 0..N {
        for j in 0..N {
            res[i] += a[i][j] * vec[j];
        }
    }
    res
}

fn transpose<const N: usize>(mat: &[[isize; N]; N]) -> [[isize; N]; N] {
    let mut res = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            res[j][i] = mat[i][j];
        }
    }
    res
}

fn invert_vec(direction: [isize; 2]) -> [isize; 2] {
    [-direction[0], -direction[1]]
}
