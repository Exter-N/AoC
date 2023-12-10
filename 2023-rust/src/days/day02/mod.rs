use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{map, opt},
    multi::fold_many1,
    sequence::{preceded, separated_pair, terminated},
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn with(self, color: Color, number: u32) -> Self {
        match color {
            Color::Red => Self {
                red: number,
                ..self
            },
            Color::Green => Self {
                green: number,
                ..self
            },
            Color::Blue => Self {
                blue: number,
                ..self
            },
        }
    }
    fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn fits_within(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

struct Day2 {
    sum_of_possible: u32,
    sum_of_powers: u32,
}

impl Day2 {
    fn new() -> Self {
        Self {
            sum_of_possible: 0,
            sum_of_powers: 0,
        }
    }
}

static REFERENCE_CUBES: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

impl LineStreamHandler for Day2 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (id, cubes) = parse_full_string(
            line,
            separated_pair(
                preceded(tag("Game "), u32),
                tag(": "),
                fold_many1(
                    terminated(
                        fold_many1(
                            terminated(
                                separated_pair(
                                    u32,
                                    tag(" "),
                                    alt((
                                        map(tag("red"), |_| Color::Red),
                                        map(tag("green"), |_| Color::Green),
                                        map(tag("blue"), |_| Color::Blue),
                                    )),
                                ),
                                opt(tag(", ")),
                            ),
                            CubeSet::new,
                            |previous, (number, color)| previous.with(color, number),
                        ),
                        opt(tag("; ")),
                    ),
                    CubeSet::new,
                    |previous, current| previous.max(current),
                ),
            ),
        )?;
        if cubes.fits_within(&REFERENCE_CUBES) {
            self.sum_of_possible += id;
        }
        self.sum_of_powers += cubes.power();
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of IDs of possible games: {}",
            SILVER_ANSI, self.sum_of_possible
        );
        println!(
            "[{}] Sum of minimal set powers:    {}",
            GOLD_ANSI, self.sum_of_powers
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(2, "Cube Conundrum", Day2::new()))
}
