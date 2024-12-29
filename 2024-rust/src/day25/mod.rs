use std::error::Error;

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

const COLUMNS: usize = 5;
const ROWS: u8 = 5;

#[derive(Debug)]
struct Day25 {
    locks: Vec<[u8; COLUMNS]>,
    keys: Vec<[u8; COLUMNS]>,
    current_row: usize,
    current_is_key: bool,
    current: [u8; COLUMNS],
}

impl Day25 {
    fn new() -> Self {
        Self {
            locks: Vec::new(),
            keys: Vec::new(),
            current_row: 0,
            current_is_key: false,
            current: [u8::MAX; COLUMNS],
        }
    }

    fn flush(&mut self) {
        if self.current_row == 0 {
            return;
        }

        if self.current_is_key {
            self.keys.push(self.current);
        } else {
            self.locks.push(self.current);
        }
        self.current_row = 0;
        self.current_is_key = false;
        self.current = [u8::MAX; COLUMNS];
    }
}

fn fits(lock: &[u8; COLUMNS], key: &[u8; COLUMNS]) -> bool {
    for i in 0..COLUMNS {
        if lock[i] + key[i] > ROWS {
            return false;
        }
    }

    true
}

impl LineStreamHandler for Day25 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            self.flush();
            return Ok(());
        }

        if self.current_row == 0 {
            self.current_is_key = line.chars().next() == Some('.');
        } else if self.current_is_key {
            for (ch, i) in line.chars().zip(0..COLUMNS) {
                if ch == '#' && self.current[i] == u8::MAX {
                    self.current[i] = ROWS + 1 - (self.current_row as u8);
                }
            }
        } else {
            for (ch, i) in line.chars().zip(0..COLUMNS) {
                if ch == '.' && self.current[i] == u8::MAX {
                    self.current[i] = (self.current_row as u8) - 1;
                }
            }
        }
        self.current_row += 1;
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.flush();
        let mut pairs = 0usize;
        for key in self.keys.iter() {
            for lock in self.locks.iter() {
                if fits(lock, key) {
                    pairs += 1;
                }
            }
        }
        println!("[{}] Fitting lock/key pairs: {}", SILVER_ANSI, pairs);
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(25, "Code Chronicle", Day25::new()))
}
