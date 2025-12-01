use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{anychar, u32};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandlerOnce},
};

mod state;

use state::ShipWithCrane;

struct Day5Stacks(ShipWithCrane);

struct Day5Moves(ShipWithCrane);

impl LineStreamHandlerOnce for Day5Stacks {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(Box::new(Day5Moves(self.0)));
        }

        let crates = parse_full_string(
            line,
            separated_list1(anychar, delimited(anychar, anychar, anychar)),
        )?;

        self.0.add_bottom_layer(crates);

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

impl LineStreamHandlerOnce for Day5Moves {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let (num, from, to) = parse_full_string(
            line,
            (
                preceded(tag("move "), u32),
                preceded(tag(" from "), u32),
                preceded(tag(" to "), u32),
            ),
        )?;
        self.0
            .move_top(num as usize, from as usize - 1, to as usize - 1)?;

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let state = self.0;
        println!(
            "[{}] Stack tops: {}",
            if state.reverse_on_move {
                SILVER_ANSI
            } else {
                GOLD_ANSI
            },
            state.tops()?
        );
        if state.verbose {
            state.dump();
        }

        Ok(())
    }
}

pub fn new(gold: bool, verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(
        5,
        "Supply Stacks",
        Day5Stacks(ShipWithCrane::new(!gold, verbose)),
    ))
}
