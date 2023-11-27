use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use nom::bytes::complete::tag;
use nom::character::complete::{anychar, u32};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};

use crate::days::{GOLD_ANSI, SILVER_ANSI};

use super::{parse_full_string, LineStreamHandler};

mod state;

use state::ShipWithCrane;

struct Day5Stacks(Rc<RefCell<ShipWithCrane>>);

struct Day5Moves(Rc<RefCell<ShipWithCrane>>);

impl LineStreamHandler for Day5Stacks {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(Some(Box::new(Day5Moves(self.0.clone()))));
        }

        let crates = parse_full_string(
            line,
            separated_list1(anychar, delimited(anychar, anychar, anychar)),
        )?;

        self.0.borrow_mut().add_bottom_layer(crates);

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

impl LineStreamHandler for Day5Moves {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let (num, from, to) = parse_full_string(
            line,
            tuple((
                preceded(tag("move "), u32),
                preceded(tag(" from "), u32),
                preceded(tag(" to "), u32),
            )),
        )?;
        self.0
            .borrow_mut()
            .move_top(num as usize, from as usize - 1, to as usize - 1)?;

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        let state = self.0.borrow();
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

pub fn new(
    gold: bool,
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        5,
        "Supply Stacks",
        Box::new(Day5Stacks(ShipWithCrane::new(!gold, verbose))),
    ))
}
