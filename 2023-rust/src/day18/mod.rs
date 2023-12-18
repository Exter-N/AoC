use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    point::{Direction2, Point2},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, hex_digit1, multispace1, u32},
    combinator::{map, map_res, value},
    sequence::{delimited, separated_pair},
};
use std::error::Error;

fn area(perimeter: &Vec<(Direction2, usize)>) -> Result<usize, Box<dyn Error>> {
    let mut head: Point2<isize> = Point2(0, 0);
    let mut sum_of_determinants: isize = 0;
    let mut length_of_perimeter = 0;
    for (towards, distance) in perimeter {
        let new_head = head.towards(*towards, *distance as isize);
        sum_of_determinants += head.0 * new_head.1 - head.1 * new_head.0;
        head = new_head;
        length_of_perimeter += distance;
    }
    if head != Point2(0, 0) {
        return Err("unclosed loop".into());
    }
    Ok(sum_of_determinants.abs() as usize / 2 + length_of_perimeter / 2 + 1)
}

struct Day18 {
    gold: bool,
    perimeter: Vec<(Direction2, usize)>,
}

impl Day18 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            perimeter: Vec::new(),
        }
    }
}

impl LineStreamHandler for Day18 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (vector1, vector2) = parse_full_string(
            line,
            separated_pair(
                separated_pair(
                    alt((
                        value(Direction2::Right, char('R')),
                        value(Direction2::Down, char('D')),
                        value(Direction2::Left, char('L')),
                        value(Direction2::Up, char('U')),
                    )),
                    multispace1,
                    map(u32, |num| num as usize),
                ),
                multispace1,
                delimited(
                    tag("(#"),
                    map_res(hex_digit1, |num| {
                        let hex_code = u32::from_str_radix(num, 16)?;
                        Ok::<_, Box<dyn Error>>((
                            Direction2::try_from((hex_code & 15) as u8)?,
                            (hex_code >> 4) as usize,
                        ))
                    }),
                    char(')'),
                ),
            ),
        )?;
        let vector = if self.gold { vector2 } else { vector1 };
        self.perimeter.push(vector);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Trench area: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            area(&self.perimeter)?
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(18, "Lavaduct Lagoon", Day18::new(gold)))
}
