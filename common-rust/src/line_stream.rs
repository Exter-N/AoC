use std::array::TryFromSliceError;
use std::error::Error;

use nom::bytes::complete::take;
use nom::combinator::{eof, map_res};
use nom::error::{FromExternalError, ParseError};
use nom::sequence::terminated;
use nom::{Finish, Parser};

pub fn parse_full_string<'a, F>(s: &'a str, f: F) -> Result<<F as Parser<&'a str>>::Output, nom::error::Error<usize>>
where
    F: Parser<&'a str, Error = nom::error::Error<&'a str>>,
{
    match terminated(f, eof).parse(s).finish() {
        Ok((_, result)) => Ok(result),
        Err(error) => Err(nom::error::Error::new(
            (error.input.as_ptr() as usize) - (s.as_ptr() as usize),
            error.code,
        )),
    }
}

pub fn take_fixed<'a, const N: usize, E>() -> impl Parser<&'a str, Output = [u8; N], Error = E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, TryFromSliceError>,
{
    map_res(take(N), |code: &str| <[u8; N]>::try_from(code.as_bytes()))
}

pub trait LineStreamHandler {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>>;
    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}

pub trait LineStreamHandlerOnce {
    fn update(
        self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>>;
    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>>;
}

struct WrappedLineStreamHandlerOnce(Option<Box<dyn LineStreamHandlerOnce>>);

impl LineStreamHandler for WrappedLineStreamHandlerOnce {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if let Some(handler) = std::mem::take(&mut self.0) {
            self.0 = Some(handler.update(line)?);
            Ok(())
        } else {
            Err(
                "cannot continue using a wrapped LineStreamHandlerOnce that returned an error"
                    .into(),
            )
        }
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.0.unwrap().finish()
    }
}

pub fn wrap_once<H>(handler: H) -> impl LineStreamHandler
where
    H: LineStreamHandlerOnce + 'static,
{
    WrappedLineStreamHandlerOnce(Some(Box::new(handler)))
}
