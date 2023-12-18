use std::{
    collections::HashSet,
    error::Error,
    ops::{Range, Sub},
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::Point2,
};

struct Day11 {
    galaxies: HashSet<Point2<usize>>,
    empty_columns: Vec<usize>,
    empty_rows: Vec<usize>,
    width: usize,
    height: usize,
}

fn binary_search_range<T>(slice: &[T], lower: &T, upper: &T) -> Range<usize>
where
    T: Ord,
{
    (match slice.binary_search(lower) {
        Ok(n) => n,
        Err(n) => n,
    })..(match slice.binary_search(upper) {
        Ok(n) => n + 1,
        Err(n) => n,
    })
}

#[inline(always)]
fn range_width<T>(range: Range<T>) -> <T as Sub<T>>::Output
where
    T: Sub<T>,
{
    range.end - range.start
}

impl Day11 {
    fn new() -> Self {
        Self {
            galaxies: HashSet::new(),
            empty_columns: Vec::new(),
            empty_rows: Vec::new(),
            width: 0,
            height: 0,
        }
    }
    fn distance(&self, from: Point2<usize>, to: Point2<usize>) -> (usize, usize) {
        let empty_columns = range_width(binary_search_range(
            &self.empty_columns,
            &from.0.min(to.0),
            &from.0.max(to.0),
        ));
        let empty_rows = range_width(binary_search_range(
            &self.empty_rows,
            &from.1.min(to.1),
            &from.1.max(to.1),
        ));
        let empty = empty_columns + empty_rows;
        (from.manhattan_distance(to) - empty, empty)
    }
}

impl LineStreamHandler for Day11 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut is_empty_row = true;
        for (ch, i) in line.chars().zip(0usize..) {
            let is_galaxy = ch == '#';
            if is_galaxy {
                self.galaxies.insert(Point2(i, self.height));
                is_empty_row = false;
                if let Ok(column_pos) = self.empty_columns.binary_search(&i) {
                    self.empty_columns.remove(column_pos);
                }
            }
            if i == self.width {
                self.width += 1;
                if !is_galaxy {
                    self.empty_columns.push(i);
                }
            }
        }
        if is_empty_row {
            self.empty_rows.push(self.height);
        }
        self.height += 1;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut sum_of_fixed_distances = 0usize;
        let mut sum_of_expandable_distances = 0usize;
        for (galaxy1, i) in self.galaxies.iter().zip(0usize..) {
            for galaxy2 in self.galaxies.iter().skip(i + 1) {
                let (fixed_distance, expandable_distance) = self.distance(*galaxy1, *galaxy2);
                sum_of_fixed_distances += fixed_distance;
                sum_of_expandable_distances += expandable_distance;
            }
        }
        println!(
            "[-] Sum of base distances:    {}",
            sum_of_fixed_distances + sum_of_expandable_distances
        );
        println!(
            "[{}] Sum of distances (E×2):   {}",
            SILVER_ANSI,
            sum_of_fixed_distances + sum_of_expandable_distances * 2
        );
        println!(
            "[-] Sum of distances (E×10):  {}",
            sum_of_fixed_distances + sum_of_expandable_distances * 10
        );
        println!(
            "[-] Sum of distances (E×100): {}",
            sum_of_fixed_distances + sum_of_expandable_distances * 100
        );
        println!(
            "[{}] Sum of distances (E×1M):  {}",
            GOLD_ANSI,
            sum_of_fixed_distances + sum_of_expandable_distances * 1_000_000
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(11, "Cosmic Expansion", Day11::new()))
}
