use std::error::Error;

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

fn find_symmetry(vec: &Vec<u64>, up_to_error_bits: u32) -> Option<usize> {
    'outer: for i in 1..vec.len() {
        let mut error_bits = 0;
        for j in 0..i.min(vec.len() - i) {
            error_bits += (vec[i + j] ^ vec[i - 1 - j]).count_ones();
            if error_bits > up_to_error_bits {
                continue 'outer;
            }
        }
        if error_bits == up_to_error_bits {
            return Some(i);
        }
    }
    None
}

struct Day13 {
    gold: bool,
    sum_of_notes: usize,
    current_rows: Vec<u64>,
    current_columns: Vec<u64>,
}

impl Day13 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            sum_of_notes: 0,
            current_rows: Vec::new(),
            current_columns: Vec::new(),
        }
    }
    fn end_current(&mut self) -> Result<(), Box<dyn Error>> {
        let up_to_error_bits = if self.gold { 1 } else { 0 };
        match (
            find_symmetry(&self.current_rows, up_to_error_bits),
            find_symmetry(&self.current_columns, up_to_error_bits),
        ) {
            (Some(sym_row), None) => {
                self.sum_of_notes += 100 * sym_row;
            }
            (None, Some(sym_column)) => {
                self.sum_of_notes += sym_column;
            }
            (None, None) => {
                return Err("no symmetry found".into());
            }
            (Some(_), Some(_)) => {
                return Err("multiple symmetries found".into());
            }
        }
        self.current_rows.clear();
        self.current_columns.clear();
        Ok(())
    }
}

impl LineStreamHandler for Day13 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            return self.end_current();
        }
        let mut row = 0u64;
        for (ch, i) in line.chars().zip(0usize..) {
            let value = if ch == '#' { 1u64 } else { 0u64 };
            row = (row << 1) | value;
            if let Some(column) = self.current_columns.get_mut(i) {
                *column = (*column << 1) | value;
            } else {
                self.current_columns.push(value);
            }
        }
        self.current_rows.push(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.end_current()?;
        println!("[{}] Sum of notes: {}", SILVER_ANSI, self.sum_of_notes);
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(13, "Point of Incidence", Day13::new(gold)))
}
