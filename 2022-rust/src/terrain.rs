use std::ops::{Deref, DerefMut, Index, IndexMut};

use itertools::Itertools;

use crate::point::{Direction2, Point2};

pub struct Terrain<T> {
    width: usize,
    height: usize,
    terrain: Vec<Vec<T>>,
}

impl<T> Terrain<T> {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn new_row(&self) -> Vec<T> {
        Vec::with_capacity(self.width)
    }
    pub fn push_row(&mut self, row: Vec<T>) {
        if row.len() > self.width {
            self.width = row.len();
        }
        self.terrain.push(row);
        self.height += 1;
    }
    pub fn points(&self) -> impl Iterator<Item = Point2<usize>> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(i, j)| Point2(j, i))
    }
    pub fn neighbors<'a>(
        self: &'a Self,
        pt: Point2<usize>,
    ) -> impl Iterator<Item = (Direction2, Point2<usize>)> + 'a {
        Direction2::all().filter_map(move |dir| {
            if match dir {
                Direction2::Right => pt.0 + 1 < self.width,
                Direction2::Down => pt.1 + 1 < self.height,
                Direction2::Left => pt.0 >= 1,
                Direction2::Up => pt.1 >= 1,
            } {
                Some((dir, pt.next_towards(dir)))
            } else {
                None
            }
        })
    }
}

impl<T> Default for Terrain<T> {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            terrain: Default::default(),
        }
    }
}

impl<T> Deref for Terrain<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.terrain
    }
}

impl<T> DerefMut for Terrain<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terrain
    }
}

impl<T> Index<Point2<usize>> for Terrain<T> {
    type Output = T;

    fn index(&self, index: Point2<usize>) -> &Self::Output {
        &self.terrain[index.1][index.0]
    }
}

impl<T> IndexMut<Point2<usize>> for Terrain<T> {
    fn index_mut(&mut self, index: Point2<usize>) -> &mut Self::Output {
        &mut self.terrain[index.1][index.0]
    }
}
