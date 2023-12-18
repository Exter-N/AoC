use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Snafu(pub u64);

impl TryFrom<&str> for Snafu {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed: u64 = 0;
        for ch in value.chars() {
            parsed = parsed * 5
                + match ch {
                    '=' => 0,
                    '-' => 1,
                    '0' => 2,
                    '1' => 3,
                    '2' => 4,
                    _ => return Err(Box::from("invalid SNAFU character")),
                }
                - 2;
        }

        Ok(Self(parsed))
    }
}

impl From<&Snafu> for String {
    fn from(value: &Snafu) -> Self {
        let mut s: VecDeque<char> = VecDeque::new();
        let mut val = value.0;
        while val > 0 {
            s.push_front(match val % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            });
            val = (val + 2) / 5;
        }
        if s.is_empty() {
            s.push_back('0')
        }

        s.iter().collect()
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&String::from(self))
    }
}
