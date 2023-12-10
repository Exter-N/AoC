use std::{collections::HashSet, hash::Hash};

use aoc_common_rs::point::Point2;

fn do_pull_towards(
    point: Point2<i16>,
    towards: Point2<i16>,
    previous_towards: Point2<i16>,
) -> Point2<i16> {
    if point.chebyshev_distance(towards) > 1 {
        if let Some(direction) = point.direction_towards(&towards) {
            point.next_towards(direction)
        } else if towards.manhattan_distance(previous_towards) > 1 {
            point + (towards - previous_towards)
        } else {
            previous_towards
        }
    } else {
        point
    }
}

pub struct TracedPoint<T>
where
    T: Eq + Hash,
{
    pub current: Point2<T>,
    pub trace: HashSet<Point2<T>>,
}

impl<T> TracedPoint<T>
where
    T: Copy + Eq + Hash,
{
    fn new(initial: Point2<T>) -> Self {
        Self {
            current: initial,
            trace: HashSet::from([initial]),
        }
    }
    fn move_to(&mut self, new: Point2<T>) {
        self.current = new;
        self.trace.insert(new);
    }
}

impl TracedPoint<i16> {
    pub fn pull_towards(&mut self, towards: Point2<i16>, previous_towards: Point2<i16>) {
        self.move_to(do_pull_towards(self.current, towards, previous_towards))
    }
}

impl<T> Default for TracedPoint<T>
where
    T: Copy + Default + Eq + Hash,
{
    fn default() -> Self {
        Self::new(Point2::default())
    }
}

pub fn pull_towards(
    point: &mut Point2<i16>,
    towards: Point2<i16>,
    previous_towards: Point2<i16>,
) -> (Point2<i16>, Point2<i16>) {
    let previous_point = *point;
    *point = do_pull_towards(*point, towards, previous_towards);

    (*point, previous_point)
}
