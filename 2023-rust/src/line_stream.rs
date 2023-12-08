use std::array::TryFromSliceError;
use std::error::Error;

use nom::bytes::complete::take;
use nom::combinator::{eof, map_res};
use nom::error::{FromExternalError, ParseError};
use nom::sequence::terminated;
use nom::{Finish, IResult, Parser};

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

pub fn take_fixed<'a, const N: usize, E>() -> impl FnMut(&'a str) -> IResult<&'a str, [u8; N], E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, TryFromSliceError>,
{
    map_res(take(N), |code: &str| <[u8; N]>::try_from(code.as_bytes()))
}

pub trait LineStreamHandler {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>>;
    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}
