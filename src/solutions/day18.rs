use std::collections::HashSet;

use crate::utils::{add_vecs, Reader};

static ADJACENT_DELTAS: &[[isize; 3]] = &[
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

pub fn part1(input_path: &str) {
    let droplets = parse_droplets(input_path);
    let mut exposed_count = 0;
    for droplet_pos in droplets.iter() {
        for delta in ADJACENT_DELTAS {
            if !droplets.contains(&add_vecs(droplet_pos, delta)) {
                exposed_count += 1;
            }
        }
    }
    println!("{}", exposed_count);
}

pub fn part2(input_path: &str) {
    let droplets = parse_droplets(input_path);
    let (bbmin, bbmax) = bounding_box(droplets.iter()).unwrap();

    // add an additional layer so that we have enough space to surround the clump
    let min = add_vecs(&bbmin, &[-1, -1, -1]);
    let max = add_vecs(&bbmax, &[1, 1, 1]);

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    stack.push(min);
    visited.insert(min);

    let mut exposed_side_count = 0;
    while let Some(pos) = stack.pop() {
        for delta in ADJACENT_DELTAS {
            let ngbr = add_vecs(&pos, delta);
            if !vec_is_in_bounds(&ngbr, (&min, &max)) || visited.contains(&ngbr) {
                continue;
            }
            if droplets.contains(&ngbr) {
                // we have found an exposed exposed side
                exposed_side_count += 1;
            } else {
                // enqueue this empty space
                stack.push(ngbr);
                visited.insert(ngbr);
            }
        }
    }

    println!("{}", exposed_side_count);
}

fn bounding_box<'a, const N: usize, I>(mut iter: I) -> Option<([isize; N], [isize; N])>
where
    I: Iterator<Item = &'a [isize; N]>,
{
    let initial = iter.next()?;
    let mut min = *initial;
    let mut max = *initial;

    for point in iter {
        for i in 0..N {
            if point[i] < min[i] {
                min[i] = point[i];
            } else if point[i] > max[i] {
                max[i] = point[i];
            }
        }
    }
    Some((min, max))
}

fn vec_is_in_bounds<const N: usize>(vec: &[isize; N], bounds: (&[isize; N], &[isize; N])) -> bool {
    let (min, max) = bounds;
    (0..N).all(|i| vec[i] >= min[i] && vec[i] <= max[i])
}

fn parse_droplets(input_path: &str) -> HashSet<[isize; 3]> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut droplets = HashSet::new();
    while reader.has_next() {
        let droplet = read_droplet(&mut reader);
        droplets.insert(droplet);
    }
    droplets
}

fn read_droplet(reader: &mut Reader) -> [isize; 3] {
    let x = reader.read_delimited_usize(b',');
    let y = reader.read_delimited_usize(b',');
    let z = reader.read_delimited_usize(b'\n');
    [x as isize, y as isize, z as isize]
}
