use std::error::Error;

use nom::combinator::eof;
use nom::sequence::terminated;
use nom::{Finish, Parser};

pub fn parse_full_string<'a, O, F>(s: &'a str, f: F) -> Result<O, nom::error::Error<usize>>
where
    F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    match terminated(f, eof)(s).finish() {
        Ok((_, result)) => Ok(result),
        Err(error) => Err(nom::error::Error::new(
            (error.input.as_ptr() as usize) - (s.as_ptr() as usize),
            error.code,
        )),
    }
}

pub trait LineStreamHandler {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>>;
    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}
