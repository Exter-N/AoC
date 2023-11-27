use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use lazy_static::lazy_static;

use crate::point::Direction2;

use super::{LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

lazy_static! {
    static ref ROCKS: Vec<Vec<u8>> = vec![
        vec![0b0011110],
        vec![0b0001000, 0b0011100, 0b0001000],
        vec![0b0011100, 0b0000100, 0b0000100],
        vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
        vec![0b0011000, 0b0011000],
    ];
}

#[derive(Clone, Debug, Default, Eq)]
struct TowerState {
    tower: Vec<u8>,
    locked_rocks: usize,
    cleared_rows: usize,
    next_rock_index: u8,
}

impl TowerState {
    fn height(&self) -> usize {
        self.cleared_rows + self.tower.len()
    }
    fn clear_rows(&mut self, max_rows_to_keep: usize) {
        let mut mask = 0b1111111;
        let mut rows_to_clear = self.tower.len().saturating_sub(max_rows_to_keep);
        for (row, i) in self.tower.iter().zip(rows_to_clear..self.tower.len()).rev() {
            mask &= !row;
            mask = (mask | (mask << 1) | (mask >> 1)) & 0b1111111 & !row;
            if 0 == mask {
                rows_to_clear = i + 1;
                break;
            }
        }
        if rows_to_clear > 0 {
            self.tower.drain(0..rows_to_clear);
            self.cleared_rows += rows_to_clear;
        }
    }
    fn dump_tower(&self) {
        for row in self.tower.iter().rev() {
            print!("|");
            for bit in (0..7).rev() {
                print!("{}", if 0 != (row & (1 << bit)) { '#' } else { '.' });
            }
            println!("|");
        }
        if self.cleared_rows > 0 {
            println!("|{:^7}|", format!("+{}", self.cleared_rows));
        } else {
            println!("+-------+");
        }
    }
}

impl Hash for TowerState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tower.hash(state);
        self.next_rock_index.hash(state);
    }
}

impl PartialEq for TowerState {
    fn eq(&self, other: &Self) -> bool {
        self.tower == other.tower && self.next_rock_index == other.next_rock_index
    }
}

#[derive(Default)]
struct Tower {
    state: TowerState,
    max_rows_to_keep: usize,
    current_rock: Vec<u8>,
    current_y: usize,
}

impl Tower {
    fn new(max_rows_to_keep: usize) -> Self {
        let mut new = Self {
            max_rows_to_keep,
            ..Default::default()
        };
        new.generate_rock();

        new
    }
    fn snapshot(&self) -> TowerState {
        self.state.clone()
    }
    fn generate_rock(&mut self) {
        self.current_rock = ROCKS[self.next_rock_index as usize].clone();
        self.next_rock_index = (self.next_rock_index + 1) % 5;
        self.current_y = self.tower.len() + 3;
    }
    fn lock_and_generate_rock(&mut self) {
        for _ in self.tower.len()..(self.current_y + self.current_rock.len()) {
            self.tower.push(0);
        }
        for (row, y) in self
            .current_rock
            .iter()
            .zip(self.current_y..self.tower.len())
        {
            self.state.tower[y] |= row;
        }
        self.locked_rocks += 1;
        let max_rows_to_keep = self.max_rows_to_keep;
        self.clear_rows(max_rows_to_keep);
        self.generate_rock();
    }
    fn can_freely_shift_rock(&mut self, direction: Direction2) -> bool {
        match direction {
            Direction2::Right => {
                for (row, i) in self.current_rock.iter().zip(0usize..) {
                    if 0 != (row & 0b0000001)
                        || (self.current_y + i) < self.tower.len()
                            && 0 != (self.tower[self.current_y + i] & (row >> 1))
                    {
                        return false;
                    }
                }

                true
            }
            Direction2::Down => {
                if 0 == self.current_y {
                    return false;
                }
                for (row, y) in self
                    .current_rock
                    .iter()
                    .zip((self.current_y - 1)..self.tower.len())
                {
                    if 0 != (self.tower[y] & row) {
                        return false;
                    }
                }

                true
            }
            Direction2::Left => {
                for (row, i) in self.current_rock.iter().zip(0usize..) {
                    if 0 != (row & 0b1000000)
                        || (self.current_y + i) < self.tower.len()
                            && 0 != (self.tower[self.current_y + i] & (row << 1))
                    {
                        return false;
                    }
                }

                true
            }
            Direction2::Up => unimplemented!(),
        }
    }
    fn shift_rock(&mut self, direction: Direction2) {
        match direction {
            Direction2::Right => {
                if self.can_freely_shift_rock(direction) {
                    for row in &mut self.current_rock {
                        *row >>= 1;
                    }
                }
            }
            Direction2::Down => {
                if self.can_freely_shift_rock(direction) {
                    self.current_y -= 1;
                } else {
                    self.lock_and_generate_rock();
                }
            }
            Direction2::Left => {
                if self.can_freely_shift_rock(direction) {
                    for row in &mut self.current_rock {
                        *row <<= 1;
                    }
                }
            }
            Direction2::Up => unimplemented!(),
        }
    }
    fn dump_tower(&self) {
        self.snapshot().dump_tower()
    }
}

impl Deref for Tower {
    type Target = TowerState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Tower {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

#[derive(Default)]
struct Day17 {
    verbose: bool,
    target_locked_rocks: usize,
}

impl Day17 {
    fn new(target_locked_rocks: usize, verbose: bool) -> Self {
        Self {
            verbose,
            target_locked_rocks,
            ..Default::default()
        }
    }
}

impl LineStreamHandler for Day17 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let mut tower = Tower::new(max(10000, line.len()));
        let mut snapshots: HashSet<TowerState> = HashSet::new();
        'cycle: for _ in 0usize.. {
            for ch in line.chars() {
                if tower.locked_rocks >= self.target_locked_rocks {
                    break 'cycle;
                }
                match ch {
                    '<' => tower.shift_rock(Direction2::Left),
                    '>' => tower.shift_rock(Direction2::Right),
                    _ => return Err(Box::from("unrecognized char")),
                }
                tower.shift_rock(Direction2::Down);
            }
            if let Some(first) = snapshots.get(&tower) {
                let rocks_per_cycle = tower.locked_rocks - first.locked_rocks;
                let rows_per_cycle = tower.cleared_rows - first.cleared_rows;
                let full_cycles = (self.target_locked_rocks - tower.locked_rocks) / rocks_per_cycle;
                tower.locked_rocks += full_cycles * rocks_per_cycle;
                tower.cleared_rows += full_cycles * rows_per_cycle;
                snapshots.clear();
            }
            snapshots.insert(tower.snapshot());
        }

        println!(
            "[{}] Tower height after {} rocks: {}",
            if self.target_locked_rocks > 1_000_000 {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.target_locked_rocks,
            tower.height()
        );
        if self.verbose {
            tower.dump_tower();
        }

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new(
    gold: bool,
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        17,
        "Pyroclastic Flow",
        Box::new(Day17::new(
            if gold { 1_000_000_000_000 } else { 2022 },
            verbose,
        )),
    ))
}
