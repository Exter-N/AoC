use std::error::Error;

use aoc_common_rs::{
    day::Day,
    line_stream::{LineStreamHandler, parse_full_string},
};
use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of, usize},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
};

#[derive(Debug)]
struct Day12 {
    shapes: Vec<usize>,
    current_shape: usize,
    current_shape_len: usize,
    fitting_regions: usize,
}

impl Day12 {
    fn new() -> Self {
        Self {
            shapes: Vec::new(),
            current_shape: 0,
            current_shape_len: 0,
            fitting_regions: 0,
        }
    }

    fn can_fit(&self, width: usize, length: usize, quantites: Vec<usize>) -> bool {
        if quantites
            .iter()
            .zip(self.shapes.iter())
            .map(|(quantity, shape)| *quantity * shape.count_ones() as usize)
            .sum::<usize>()
            > width * length
        {
            return false;
        }
        true
    }
}

impl LineStreamHandler for Day12 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            if self.current_shape_len > 0 {
                assert!(self.current_shape_len == 9);
                self.shapes.push(self.current_shape);
                self.current_shape = 0;
                self.current_shape_len = 0;
            }
            return Ok(());
        }

        if let Ok(_) = parse_full_string(line, terminated(usize, char(':'))) {
            return Ok(());
        }

        if let Ok((shape, len)) = parse_full_string(
            line,
            fold_many1(
                one_of(".#"),
                || (0usize, 0usize),
                |(shape, len), ch| {
                    (
                        match ch {
                            '#' => (shape << 1) | 1,
                            '.' => shape << 1,
                            _ => unreachable!(),
                        },
                        len + 1,
                    )
                },
            ),
        ) {
            self.current_shape |= shape << self.current_shape_len;
            self.current_shape_len += len;
            return Ok(());
        }

        let ((width, length), quantities) = parse_full_string(
            line,
            separated_pair(
                separated_pair(usize, char('x'), usize),
                tag(": "),
                separated_list1(char(' '), usize),
            ),
        )?;

        if self.can_fit(width, length, quantities) {
            self.fitting_regions += 1;
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[-] Upper bound of fitting regions: {}",
            self.fitting_regions
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(12, "Christmas Tree Farm", Day12::new()))
}
