use std::{
    cell::RefCell,
    error::Error,
    io::{self, BufRead},
    time::Instant,
};

use crate::line_stream::{wrap_once, LineStreamHandler, LineStreamHandlerOnce};

pub const GOLD_ANSI: &str = "\x1B[38;2;255;215;0mG\x1B[m";
pub const SILVER_ANSI: &str = "\x1B[38;2;192;192;192mS\x1B[m";

pub struct Day {
    pub display_banner: bool,
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
            display_banner: true,
            number,
            title,
            handler: RefCell::new(Box::new(handler)),
        }
    }

    pub fn new_once<H>(number: u8, title: &'static str, handler: H) -> Self
    where
        H: LineStreamHandlerOnce + 'static,
    {
        Self::new(number, title, wrap_once(handler))
    }

    pub fn with_display_banner(mut self, display_banner: bool) -> Self {
        self.display_banner = display_banner;
        self
    }

    pub fn update(&self, line: &str) -> Result<(), Box<dyn Error>> {
        self.handler.borrow_mut().update(line)
    }

    pub fn finish(self) -> Result<(), Box<dyn Error>> {
        self.handler.into_inner().finish()
    }
}

pub fn run(day: Day, timed: bool) -> Result<(), Box<dyn Error>> {
    if day.display_banner {
        eprintln!("--- Day {}: {} ---", day.number, day.title);
    }

    let start_time = Instant::now();
    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    let result = day.finish();
    if timed {
        eprintln!("--- Time: {:?} ---", start_time.elapsed());
    }

    result
}
