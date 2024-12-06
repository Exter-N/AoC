use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    branch::alt,
    character::complete::{anychar, char},
    combinator::map,
    multi::fold_many1,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Letter {
    X,
    M,
    A,
    S,
    Invalid,
}

fn count_horizontals(row: &Vec<Letter>) -> usize {
    use Letter::*;

    let mut occurrences = 0usize;
    for (letter, i) in row.iter().zip(0..(row.len() - 3)) {
        match *letter {
            X => {
                if row[i + 1] == M && row[i + 2] == A && row[i + 3] == S {
                    occurrences += 1;
                }
            }
            S => {
                if row[i + 1] == A && row[i + 2] == M && row[i + 3] == X {
                    occurrences += 1;
                }
            }
            _ => {}
        }
    }
    occurrences
}

fn count_non_horizontals(window: &[Option<Vec<Letter>>], shift: usize) -> usize {
    use Letter::*;

    let mut occurrences = 0usize;
    let row0 = window[shift].as_ref().unwrap();
    let row1 = window[(shift + 1) % window.len()].as_ref().unwrap();
    let row2 = window[(shift + 2) % window.len()].as_ref().unwrap();
    let row3 = window[(shift + 3) % window.len()].as_ref().unwrap();

    for (letter, i) in row0.iter().zip(0usize..) {
        // Verticals
        match *letter {
            X => {
                if row1[i] == M && row2[i] == A && row3[i] == S {
                    occurrences += 1;
                }
            }
            S => {
                if row1[i] == A && row2[i] == M && row3[i] == X {
                    occurrences += 1;
                }
            }
            _ => {}
        }
    }

    for (letter, i) in row0.iter().zip(0..(row0.len() - 3)) {
        // Rightward diagonals
        match *letter {
            X => {
                if row1[i + 1] == M && row2[i + 2] == A && row3[i + 3] == S {
                    occurrences += 1;
                }
            }
            S => {
                if row1[i + 1] == A && row2[i + 2] == M && row3[i + 3] == X {
                    occurrences += 1;
                }
            }
            _ => {}
        }
    }

    for (letter, i) in row0[3..].iter().zip(3usize..) {
        // Leftward diagonals
        match *letter {
            X => {
                if row1[i - 1] == M && row2[i - 2] == A && row3[i - 3] == S {
                    occurrences += 1;
                }
            }
            S => {
                if row1[i - 1] == A && row2[i - 2] == M && row3[i - 3] == X {
                    occurrences += 1;
                }
            }
            _ => {}
        }
    }

    occurrences
}

fn count_crosses(window: &[Option<Vec<Letter>>], shift: usize) -> usize {
    use Letter::*;

    let mut crosses = 0usize;
    let row0 = window[shift].as_ref().unwrap();
    let row1 = window[(shift + 1) % window.len()].as_ref().unwrap();
    let row2 = window[(shift + 2) % window.len()].as_ref().unwrap();

    for (letter, i) in row0.iter().zip(0..(row0.len() - 2)) {
        if row1[i + 1] == A
            && (*letter == M && row2[i + 2] == S || *letter == S && row2[i + 2] == M)
            && (row0[i + 2] == M && row2[i] == S || row0[i + 2] == S && row2[i] == M)
        {
            crosses += 1;
        }
    }

    crosses
}

#[derive(Debug)]
struct Day4 {
    window: [Option<Vec<Letter>>; 4],
    next_index: usize,
    width: usize,
    occurrences: usize,
    crosses: usize,
}

impl Day4 {
    fn new() -> Self {
        Self {
            window: [None, None, None, None],
            next_index: 0,
            width: 0,
            occurrences: 0,
            crosses: 0,
        }
    }
}

impl LineStreamHandler for Day4 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let row = parse_full_string(
            line,
            fold_many1(
                alt((
                    map(char('X'), |_| Letter::X),
                    map(char('M'), |_| Letter::M),
                    map(char('A'), |_| Letter::A),
                    map(char('S'), |_| Letter::S),
                    map(anychar, |_| Letter::Invalid),
                )),
                || Vec::with_capacity(self.width),
                |mut acc, letter| {
                    acc.push(letter);
                    acc
                },
            ),
        )?;
        self.width = self.width.max(row.len());
        self.occurrences += count_horizontals(&row);
        self.window[self.next_index] = Some(row);
        self.next_index = (self.next_index + 1) % self.window.len();
        if self.window[self.next_index].is_some() {
            self.occurrences += count_non_horizontals(&self.window, self.next_index);
            self.crosses += count_crosses(&self.window, self.next_index);
        }
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.next_index = (self.next_index + 1) % self.window.len();
        if self.window[self.next_index].is_some() {
            self.crosses += count_crosses(&self.window, self.next_index);
        }
        println!("[{}] Number of XMAS: {}", SILVER_ANSI, self.occurrences);
        println!("[{}] Number of X-MAS: {}", GOLD_ANSI, self.crosses);
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(4, "Ceres Search", Day4::new()))
}
