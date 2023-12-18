use std::cmp::{max, min, Ordering};
use std::ops::{Add, Deref, Div, RangeInclusive, Sub};

use num_traits::{one, zero, One, PrimInt, Zero};

use crate::math::is_integer;
use crate::unwrap_either;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct MultiRangeInclusive<T>(Vec<RangeInclusive<T>>);

fn range_cmp<T>(range: &RangeInclusive<T>, value: &T) -> Ordering
where
    T: Ord,
{
    if value < range.start() {
        Ordering::Greater
    } else if value > range.end() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn range_union<T>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> RangeInclusive<T>
where
    T: Copy + Ord,
{
    min(*range1.start(), *range2.start())..=max(*range1.end(), *range2.end())
}

impl<T> MultiRangeInclusive<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: Ord,
{
    pub fn contains(&self, value: &T) -> bool {
        self.0
            .binary_search_by(|range| range_cmp(range, value))
            .is_ok()
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: Copy + Ord,
{
    pub fn retain(&mut self, value: RangeInclusive<T>) {
        match self
            .0
            .binary_search_by(|range| range_cmp(range, value.end()))
        {
            Ok(end_pos) => {
                self.0.drain((end_pos + 1)..);
                self.0[end_pos] = (*self.0[end_pos].start())..=(*value.end());
            }
            Err(end_pos) => {
                self.0.drain(end_pos..);
            }
        }
        let start_pos = match self
            .0
            .binary_search_by(|range| range_cmp(range, value.start()))
        {
            Ok(start_pos) => {
                self.0[start_pos] = (*value.start())..=(*self.0[start_pos].end());
                start_pos
            }
            Err(start_pos) => start_pos,
        };
        self.0.drain(0..start_pos);
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: Copy + Div<T, Output = T> + One + Ord + PartialEq + Zero,
{
    pub fn insert(&mut self, value: RangeInclusive<T>) {
        let start_pos = match (
            self.0
                .binary_search_by(|range| range_cmp(range, value.start())),
            self.0
                .binary_search_by(|range| range_cmp(range, value.end())),
        ) {
            (Ok(start_pos), Ok(end_pos)) => {
                self.0.splice(
                    start_pos..=end_pos,
                    [range_union(&self.0[start_pos], &self.0[end_pos])],
                );
                start_pos
            }
            (Ok(start_pos), Err(end_pos)) => {
                self.0.splice(
                    start_pos..end_pos,
                    [range_union(&self.0[start_pos], &value)],
                );
                start_pos
            }
            (Err(start_pos), Ok(end_pos)) => {
                self.0
                    .splice(start_pos..=end_pos, [range_union(&value, &self.0[end_pos])]);
                start_pos
            }
            (Err(start_pos), Err(end_pos)) => {
                self.0.splice(start_pos..end_pos, [value]);
                start_pos
            }
        };
        if is_integer::<T>() {
            if start_pos + 1 < self.0.len() {
                self.merge_contiguous(start_pos + 1);
            }
            if start_pos > 0 {
                self.merge_contiguous(start_pos);
            }
        }
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: Copy + Div<T, Output = T> + One + Ord + PartialEq + Sub<T, Output = T> + Zero,
{
    pub fn remove(&mut self, value: RangeInclusive<T>) {
        let mut start = self
            .0
            .binary_search_by(|range| range_cmp(range, value.start()));
        let mut end = self
            .0
            .binary_search_by(|range| range_cmp(range, value.end()));
        if let Ok(start_pos) = start {
            if let Ok(end_pos) = end {
                if start_pos == end_pos
                    && value.start() != self.0[start_pos].start()
                    && value.end() != self.0[end_pos].end()
                {
                    self.0.insert(
                        end_pos + 1,
                        (*value.end() + one::<T>())..=(*self.0[end_pos].end()),
                    );
                    self.0[start_pos] =
                        (*self.0[start_pos].start())..=(*value.start() - one::<T>());
                    return;
                }
            }
        }
        if let Ok(end_pos) = end {
            if value.end() == self.0[end_pos].end() {
                if let Ok(start_pos) = start {
                    if start_pos != end_pos {
                        end = Ok(end_pos + 1);
                    }
                } else {
                    end = Ok(end_pos + 1);
                }
            } else {
                self.0[end_pos] = (*value.end() + one::<T>())..=(*self.0[end_pos].end());
                if let Ok(start_pos) = start {
                    if start_pos == end_pos {
                        start = Err(start_pos);
                    }
                }
            }
        }
        if let Ok(start_pos) = start {
            if value.start() != self.0[start_pos].start() {
                self.0[start_pos] = (*self.0[start_pos].start())..=(*value.start() - one::<T>());
                start = Ok(start_pos + 1);
            }
        }
        {
            let start_pos = unwrap_either(start);
            let end_pos = unwrap_either(end);
            if start_pos < end_pos {
                self.0.drain(start_pos..end_pos);
            }
        }
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: Add<T, Output = T> + Copy + One + PartialEq,
{
    fn merge_contiguous(&mut self, position: usize) {
        if *self.0[position - 1].end() + one::<T>() == *self.0[position].start() {
            let start = *self.0[position - 1].start();
            let end = *self.0.remove(position).end();
            self.0[position - 1] = start..=end;
        }
    }
}

impl<T> MultiRangeInclusive<T>
where
    T: PrimInt,
{
    pub fn count(&self) -> T {
        let mut count: T = zero();
        for range in self.0.iter() {
            count = count + *range.end() + one::<T>() - *range.start();
        }

        count
    }
}

impl<T> Deref for MultiRangeInclusive<T> {
    type Target = Vec<RangeInclusive<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Default for MultiRangeInclusive<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
