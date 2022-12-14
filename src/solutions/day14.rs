use std::{
    cmp::{max, min},
    iter,
};

use crate::utils::{GridCoords, Reader, VecGrid};

const SAND_SOURCE: GridCoords = (500, 0);

pub fn part1(input_path: &str) {
    let point_buf = parse_rock_formations(input_path);
    let (xmin, ymin, xmax, ymax) = bounding_box(
        point_buf
            .iter()
            .cloned()
            .flatten()
            .chain(iter::once(SAND_SOURCE)),
    )
    .unwrap();

    let width = 1 + xmax - xmin;
    let height = 1 + ymax - ymin;

    let mut grid = VecGrid::full(width, height, false);
    draw_rock_formations(&mut grid, (xmin, ymin), point_buf.into_iter());

    let sand_source = coords_relative_to((xmin, ymin), SAND_SOURCE);
    let count = find_abyss(&mut grid, sand_source);
    println!("{}", count);
}

fn parse_rock_formations(input_path: &str) -> Vec<Option<GridCoords>> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut point_buf = Vec::new();
    while reader.has_next() {
        let x = reader.read_usize();
        reader.skip_lit(b",");
        let y = reader.read_usize();
        point_buf.push(Some((x, y)));
        match reader.peek() {
            b'\n' => {
                reader.skip_lit(b"\n");
                point_buf.push(None);
            }
            b' ' => reader.skip_lit(b" -> "),
            _ => (),
        }
    }
    point_buf
}

fn draw_rock_formations<I>(grid: &mut VecGrid<bool>, grid_offset: GridCoords, points: I)
where
    I: Iterator<Item = Option<GridCoords>>,
{
    let (xmin, ymin) = grid_offset;
    let mut prev = None;
    for item in points {
        if let (Some((x, y)), Some((xprev, yprev))) = (item, prev) {
            if x == xprev {
                for i in min(y, yprev)..=max(y, yprev) {
                    grid[(x - xmin, i - ymin)] = true;
                }
            }
            if y == yprev {
                for i in min(x, xprev)..=max(x, xprev) {
                    grid[(i - xmin, y - ymin)] = true;
                }
            }
        }
        prev = item;
    }
}

pub fn part2(input_path: &str) {
    let point_buf = parse_rock_formations(input_path);

    let (mut xmin, ymin, mut xmax, mut ymax) = bounding_box(
        point_buf
            .iter()
            .cloned()
            .flatten()
            .chain(iter::once(SAND_SOURCE)),
    )
    .unwrap();

    ymax += 2;
    // add padding of `ymax` to both sides - this should be enough to accomodate a full slope on
    // both sides.
    xmin = min(xmin, xmin - ymax);
    xmax = max(xmax, xmax + ymax);

    let width = 1 + xmax - xmin;
    let height = 1 + ymax - ymin;

    let mut grid = VecGrid::full(width, height, false);
    draw_rock_formations(&mut grid, (xmin, ymin), point_buf.into_iter());

    for i in 0..width {
        grid[(i, ymax)] = true;
    }

    let sand_source = coords_relative_to((xmin, ymin), SAND_SOURCE);
    let count = fill_grid(&mut grid, sand_source);
    println!("{}", count);
}

fn find_abyss(grid: &mut VecGrid<bool>, start: GridCoords) -> usize {
    let mut path = Vec::with_capacity(grid.height());
    path.push(start);
    let mut count = 0;
    while fall(grid, &mut path, true) {
        let resting_position = path.pop().unwrap();
        grid[resting_position] = true;
        count += 1;
    }
    count
}

fn fill_grid(grid: &mut VecGrid<bool>, source: GridCoords) -> usize {
    let mut path = Vec::with_capacity(grid.height());
    path.push(source);
    let mut count = 0;
    while !path.is_empty() {
        fall(grid, &mut path, false);
        let resting_position = path.pop().unwrap();
        grid[resting_position] = true;
        count += 1;
    }
    count
}

// returns true if a resting position was found,
// false if this path leads to abyss
fn fall(grid: &VecGrid<bool>, path: &mut Vec<GridCoords>, check_abyss: bool) -> bool {
    let (mut x, mut y) = path.last().unwrap();
    loop {
        // try to go down
        if y + 1 >= grid.height() {
            // abyss
            return false;
        } else if check_abyss && !grid[(x, y + 1)] {
            // move down
            y += 1;
            path.push((x, y));
        } else if check_abyss && x == 0 {
            //abyss
            return false;
        } else if !grid[(x - 1, y + 1)] {
            // move diagonal left
            x -= 1;
            y += 1;
            path.push((x, y));
        } else if check_abyss && x >= grid.width() - 1 {
            // abyss
            return false;
        } else if !grid[(x + 1, y + 1)] {
            // move diagonal right
            x += 1;
            y += 1;
            path.push((x, y));
        } else {
            // found resting resting position
            return true;
        }
    }
}

// (xmin, ymin), (xmax, ymax)
fn bounding_box<I>(mut iter: I) -> Option<(usize, usize, usize, usize)>
where
    I: Iterator<Item = (usize, usize)>,
{
    let (x, y) = iter.next()?;
    let mut xmin = x;
    let mut xmax = x;
    let mut ymin = y;
    let mut ymax = y;

    for (x, y) in iter {
        if x < xmin {
            xmin = x;
        } else if x > xmax {
            xmax = x;
        }
        if y < ymin {
            ymin = y;
        } else if y > ymax {
            ymax = y;
        }
    }

    Some((xmin, ymin, xmax, ymax))
}

fn coords_relative_to(origin: GridCoords, coords: GridCoords) -> GridCoords {
    (coords.0 - origin.0, coords.1 - origin.1)
}
