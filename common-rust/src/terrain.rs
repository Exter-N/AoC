use std::ops::{Deref, DerefMut, Index, IndexMut};

use itertools::Itertools;

use crate::point::{Direction2, Point2};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Terrain<T> {
    width: usize,
    height: usize,
    terrain: Vec<Vec<T>>,
}

impl<T> Terrain<T> {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            terrain: Vec::new(),
        }
    }
    pub fn new_with(
        width: usize,
        height: usize,
        mut generator: impl FnMut(Point2<usize>) -> T,
    ) -> Self {
        let mut terrain = Self {
            width,
            height,
            terrain: Vec::with_capacity(height),
        };
        for y in 0usize..height {
            let mut row = terrain.new_row();
            for x in 0usize..width {
                row.push(generator(Point2(x, y)));
            }
            terrain.terrain.push(row);
        }
        terrain
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn size(&self) -> Point2<usize> {
        Point2(self.width, self.height)
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
    pub fn points_rev(&self) -> impl Iterator<Item = Point2<usize>> {
        (0..self.height)
            .rev()
            .cartesian_product((0..self.width).rev())
            .map(|(i, j)| Point2(j, i))
    }
    pub fn neighbors(
        &self,
        pt: Point2<usize>,
    ) -> impl Iterator<Item = (Direction2, Point2<usize>)> {
        let width = self.width;
        let height = self.height;
        Direction2::all().filter_map(move |dir| {
            if match dir {
                Direction2::Right => pt.0 + 1 < width,
                Direction2::Down => pt.1 + 1 < height,
                Direction2::Left => pt.0 >= 1,
                Direction2::Up => pt.1 >= 1,
            } {
                Some((dir, pt.next_towards(dir)))
            } else {
                None
            }
        })
    }
    pub fn walk_until(
        &self,
        from: Point2<usize>,
        from_inclusive: bool,
        towards: Direction2,
        mut predicate: impl FnMut(Point2<usize>) -> bool,
    ) -> Option<Point2<usize>> {
        if from_inclusive && predicate(from) {
            return Some(from);
        }
        match towards {
            Direction2::Right => {
                if from.0 >= self.width {
                    return None;
                }
                let mut tx = from.0 + 1;
                while tx < self.width {
                    if predicate(Point2(tx, from.1)) {
                        return Some(Point2(tx, from.1));
                    }
                    tx += 1;
                }
            }
            Direction2::Down => {
                if from.1 >= self.height {
                    return None;
                }
                let mut ty = from.1 + 1;
                while ty < self.height {
                    if predicate(Point2(from.0, ty)) {
                        return Some(Point2(from.0, ty));
                    }
                    ty += 1;
                }
            }
            Direction2::Left => {
                let mut tx = from.0;
                while tx > 0 {
                    tx -= 1;
                    if predicate(Point2(tx, from.1)) {
                        return Some(Point2(tx, from.1));
                    }
                }
            }
            Direction2::Up => {
                let mut ty = from.1;
                while ty > 0 {
                    ty -= 1;
                    if predicate(Point2(from.0, ty)) {
                        return Some(Point2(from.0, ty));
                    }
                }
            }
        }
        None
    }
    pub fn walk_while(
        &self,
        from: Point2<usize>,
        towards: Direction2,
        mut predicate: impl FnMut(Point2<usize>) -> bool,
    ) -> Point2<usize> {
        match towards {
            Direction2::Right => {
                if from.0 >= self.width {
                    return from;
                }
                let mut tx = from.0;
                let mut new_tx = tx + 1;
                while new_tx < self.width && predicate(Point2(new_tx, from.1)) {
                    tx = new_tx;
                    new_tx += 1;
                }
                Point2(tx, from.1)
            }
            Direction2::Down => {
                if from.1 >= self.height {
                    return from;
                }
                let mut ty = from.1;
                let mut new_ty = ty + 1;
                while new_ty < self.height && predicate(Point2(from.0, new_ty)) {
                    ty = new_ty;
                    new_ty += 1;
                }
                Point2(from.0, ty)
            }
            Direction2::Left => {
                let mut tx = from.0;
                while tx > 0 && predicate(Point2(tx - 1, from.1)) {
                    tx -= 1;
                }
                Point2(tx, from.1)
            }
            Direction2::Up => {
                let mut ty = from.1;
                while ty > 0 && predicate(Point2(from.0, ty - 1)) {
                    ty -= 1;
                }
                Point2(from.0, ty)
            }
        }
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
