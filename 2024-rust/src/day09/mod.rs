use std::{cmp::Ordering, error::Error, num::NonZeroUsize};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

#[derive(Clone, Copy, Debug)]
struct Run {
    length: NonZeroUsize,
    contents: Option<usize>,
}

impl Run {
    fn new(length: NonZeroUsize, contents: Option<usize>) -> Self {
        Self { length, contents }
    }
    fn lengthen(self, length_diff: usize) -> Self {
        Self {
            length: self.length.checked_add(length_diff).unwrap(),
            ..self
        }
    }
    fn shorten(self, length_diff: usize) -> Option<Self> {
        NonZeroUsize::new(self.length.get() - length_diff).map(|length| Self { length, ..self })
    }
}

fn sum_of_range(start: usize, length: usize) -> usize {
    (((start << 1) + length - 1) * length) >> 1
}

#[derive(Debug)]
struct Day9 {
    fragmenting: bool,
    disk: Vec<Run>,
    next_is_full: bool,
    next_id: usize,
}

impl Day9 {
    fn new(fragmenting: bool) -> Self {
        Self {
            fragmenting,
            disk: Vec::new(),
            next_is_full: true,
            next_id: 0,
        }
    }

    fn optimize(&mut self) {
        for i in (1..self.disk.len()).rev() {
            if self.disk[i].contents == self.disk[i - 1].contents {
                self.disk[i - 1] = self.disk[i - 1].lengthen(self.disk.remove(i).length.get());
            }
        }
    }

    fn trim_end(&mut self) {
        while let Some(last_run) = self.disk.last() {
            if last_run.contents.is_some() {
                break;
            }
            self.disk.pop();
        }
    }

    fn find_first_empty(&self, offset: usize, min_length: NonZeroUsize) -> Option<usize> {
        if offset >= self.disk.len() {
            return None;
        }

        (&self.disk[offset..])
            .iter()
            .zip(offset..)
            .find(|(el, _)| el.contents.is_none() && el.length >= min_length)
            .map(|(_, i)| i)
    }

    fn compact_fragmenting(&mut self) {
        self.trim_end();

        let len_one = NonZeroUsize::new(1).unwrap();
        let mut maybe_first_empty = self.find_first_empty(0usize, len_one);
        while let Some(first_empty) = maybe_first_empty {
            let empty_len = self.disk[first_empty].length;
            let last = *self.disk.last().unwrap();
            match last.length.cmp(&empty_len) {
                Ordering::Less => {
                    self.disk.pop();
                    self.disk.insert(
                        first_empty + 1,
                        self.disk[first_empty].shorten(last.length.get()).unwrap(),
                    );
                    self.disk[first_empty] = last;
                }
                Ordering::Equal => {
                    self.disk.swap_remove(first_empty);
                }
                Ordering::Greater => {
                    self.disk[first_empty] = Run::new(empty_len, last.contents);
                    *self.disk.last_mut().unwrap() = last.shorten(empty_len.get()).unwrap();
                }
            }
            self.trim_end();
            maybe_first_empty = self.find_first_empty(first_empty, len_one);
        }
    }

    fn compact_nonfragmenting(&mut self) {
        self.trim_end();

        let len_one = NonZeroUsize::new(1).unwrap();
        let mut first_empty = match self.find_first_empty(0usize, len_one) {
            Some(index) => index,
            None => {
                return;
            }
        };
        let mut i = self.disk.len();
        while i > 0 {
            i -= 1;
            let current = self.disk[i];
            if current.contents.is_none() {
                continue;
            }
            let move_to = match self.find_first_empty(first_empty, current.length) {
                Some(index) => index,
                None => {
                    continue;
                }
            };
            if move_to >= i {
                continue;
            }
            let empty_len = self.disk[move_to].length;
            match current.length.cmp(&empty_len) {
                Ordering::Less => {
                    self.disk[i] = Run::new(current.length, None);
                    self.disk.insert(
                        move_to + 1,
                        self.disk[move_to].shorten(current.length.get()).unwrap(),
                    );
                    self.disk[move_to] = current;
                    i += 1;
                }
                Ordering::Equal => {
                    self.disk.swap(i, move_to);
                }
                Ordering::Greater => unreachable!(),
            }
            self.trim_end();
            first_empty = match self.find_first_empty(0usize, len_one) {
                Some(index) => index,
                None => {
                    return;
                }
            };
            i = i.min(self.disk.len());
        }
    }

    fn checksum(&self) -> usize {
        let mut position = 0usize;
        let mut sum = 0usize;
        for run in &self.disk {
            if let Some(contents) = run.contents {
                sum += sum_of_range(position, run.length.get()) * contents;
            }
            position += run.length.get();
        }
        sum
    }
}

impl LineStreamHandler for Day9 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                if let Some(length) = NonZeroUsize::new(digit as usize) {
                    self.disk.push(Run::new(
                        length,
                        if self.next_is_full {
                            Some(self.next_id)
                        } else {
                            None
                        },
                    ));
                }
                if self.next_is_full {
                    self.next_id += 1;
                }
                self.next_is_full = !self.next_is_full;
            }
        }
        self.optimize();
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.fragmenting {
            self.compact_fragmenting();
        } else {
            self.compact_nonfragmenting();
        }
        self.optimize();
        println!(
            "[{}] Filesystem checksum: {}",
            if self.fragmenting {
                SILVER_ANSI
            } else {
                GOLD_ANSI
            },
            self.checksum()
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(9, "Disk Fragmenter", Day9::new(!gold)))
}
