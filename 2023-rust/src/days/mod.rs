use std::{cell::RefCell, error::Error};

use crate::line_stream::LineStreamHandler;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub const GOLD_ANSI: &str = "\x1B[38;2;255;215;0mG\x1B[m";
pub const SILVER_ANSI: &str = "\x1B[38;2;192;192;192mS\x1B[m";

pub struct Day {
    pub number: u8,
    pub title: &'static str,
    pub handler: RefCell<Box<dyn LineStreamHandler>>,
}

impl Day {
    pub fn new<H>(number: u8, title: &'static str, handler: H) -> Self
    where
        H: LineStreamHandler + 'static,
    {
        Self {
            number,
            title,
            handler: RefCell::new(Box::new(handler)),
        }
    }

    pub fn update(&self, line: &str) -> Result<(), Box<dyn Error>> {
        self.handler.borrow_mut().update(line)
    }

    pub fn finish(self) -> Result<(), Box<dyn Error>> {
        self.handler.into_inner().finish()
    }
}
