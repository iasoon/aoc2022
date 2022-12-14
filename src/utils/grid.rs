use std::ops::{Index, IndexMut};

pub type GridCoords = (usize, usize);

pub struct VecGrid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
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
    pub fn full(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        VecGrid {
            inner: vec![value; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}
