use std::ops::{Index, IndexMut};

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let grid = AsciiGrid::from_bytes(bytes);
    let mut is_visible = VecGrid::full(grid.width, grid.height, false);

    // north
    mark_visible(&grid, &mut is_visible, grid.width, grid.height, |w, d| {
        (w, d)
    });
    // south
    mark_visible(&grid, &mut is_visible, grid.width, grid.height, |w, d| {
        (w, grid.height - 1 - d)
    });
    // east
    mark_visible(&grid, &mut is_visible, grid.width, grid.height, |w, d| {
        (d, w)
    });
    // west
    mark_visible(&grid, &mut is_visible, grid.width, grid.height, |w, d| {
        (grid.width - 1 - d, w)
    });

    let num_visible: usize = is_visible.inner.iter().filter(|&&v| v).count();

    println!("{}", num_visible);
}

fn mark_visible<F>(
    grid: &AsciiGrid,
    visible: &mut VecGrid<bool>,
    width: usize,
    depth: usize,
    coord_fn: F,
) where
    F: Fn(usize, usize) -> (usize, usize),
{
    for w in 0..width {
        let mut current_height = 0;
        for d in 0..depth {
            let coord = coord_fn(w, d);
            let height = grid[coord];
            if height > current_height {
                // println!("marked visible: {:?} (height {})", coord, grid[coord]);
                current_height = height;
                visible[coord] = true;
            }
        }
    }
}

pub fn part2(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let grid = AsciiGrid::from_bytes(bytes);

    let best_score: usize = (0..grid.width)
        .flat_map(|x| (0..grid.height).map(move |y| (x, y)))
        .map(|(x, y)| tree_scenic_score(&grid, x, y))
        .max()
        .unwrap();

    println!("{}", best_score);
}

// JUST HORRENDOUS
fn tree_scenic_score(grid: &AsciiGrid, x: usize, y: usize) -> usize {
    let mut score = 1;
    let height = grid[(x, y)];

    let mut i = x;
    while i > 0 {
        i -= 1;
        if grid[(i, y)] >= height {
            break;
        }
    }
    score *= x - i;

    let mut i = x;
    while i < grid.width - 1 {
        i += 1;
        if grid[(i, y)] >= height {
            break;
        }
    }
    score *= i - x;

    let mut i = y;
    while i > 0 {
        i -= 1;
        if grid[(x, i)] >= height {
            break;
        }
    }
    score *= y - i;

    let mut i = y;
    while i < grid.height - 1 {
        i += 1;
        if grid[(x, i)] >= height {
            break;
        }
    }

    score *= i - y;

    score
}

type GridCoords = (usize, usize);

struct VecGrid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> Index<GridCoords> for VecGrid<T> {
    type Output = T;

    fn index(&self, (x, y): GridCoords) -> &Self::Output {
        &self.inner[x + self.width * y]
    }
}

impl<T> IndexMut<GridCoords> for VecGrid<T> {
    fn index_mut(&mut self, (x, y): GridCoords) -> &mut Self::Output {
        &mut self.inner[x + self.width * y]
    }
}

impl<T> VecGrid<T> {
    fn full(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        VecGrid {
            inner: vec![value; width * height],
            width,
        }
    }
}

struct AsciiGrid {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
    line_width: usize,
}

impl Index<GridCoords> for AsciiGrid {
    type Output = u8;

    fn index(&self, (x, y): GridCoords) -> &Self::Output {
        &self.bytes[x + self.line_width * y]
    }
}

impl AsciiGrid {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let width = find_first_newline(&bytes);
        let line_width = width + 1;
        let height = bytes.len() / line_width;
        AsciiGrid {
            bytes,
            width,
            height,
            line_width,
        }
    }
}

fn find_first_newline(bytes: &[u8]) -> usize {
    for (i, &c) in bytes.iter().enumerate() {
        if c == b'\n' {
            return i;
        }
    }

    panic!("no newline found")
}
