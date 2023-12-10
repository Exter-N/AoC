use std::{collections::HashMap, error::Error};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value},
    multi::many1,
    sequence::{separated_pair, terminated},
};

use aoc_common_rs::{
    cc::ThreeCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, take_fixed, LineStreamHandler},
    math::lcm,
};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Node(ThreeCC, ThreeCC);

impl Node {
    fn get(&self, direction: Direction) -> ThreeCC {
        match direction {
            Direction::Left => self.0,
            Direction::Right => self.1,
        }
    }
}

const START_POSITION: ThreeCC = ThreeCC::new('A', 'A', 'A');
const END_POSITION: ThreeCC = ThreeCC::new('Z', 'Z', 'Z');

enum Line {
    Directions(Vec<Direction>),
    Node(ThreeCC, Node),
}

struct Day8 {
    gold: bool,
    directions: Vec<Direction>,
    network: HashMap<ThreeCC, Node>,
}

impl Day8 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            directions: Vec::new(),
            network: HashMap::new(),
        }
    }
    fn moves(
        &self,
        from: ThreeCC,
        to_predicate: impl Fn(ThreeCC) -> bool,
    ) -> Result<usize, Box<dyn Error>> {
        let mut position = from;
        let mut moves = 0usize;
        for direction in self.directions.iter().cycle() {
            if let Some(current) = self.network.get(&position) {
                position = current.get(*direction);
                moves += 1;
                if to_predicate(position) {
                    break;
                }
            } else {
                return Err("node not found".into());
            }
        }
        Ok(moves)
    }
}

impl LineStreamHandler for Day8 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            return Ok(());
        }

        match parse_full_string(
            line,
            alt((
                map(
                    separated_pair(
                        map(take_fixed::<3, _>(), ThreeCC::from),
                        tag(" = ("),
                        terminated(
                            separated_pair(
                                map(take_fixed::<3, _>(), ThreeCC::from),
                                tag(", "),
                                map(take_fixed::<3, _>(), ThreeCC::from),
                            ),
                            char(')'),
                        ),
                    ),
                    |(code, node)| Line::Node(code, Node(node.0, node.1)),
                ),
                map(
                    many1(alt((
                        value(Direction::Left, char('L')),
                        value(Direction::Right, char('R')),
                    ))),
                    |directions| Line::Directions(directions),
                ),
            )),
        )? {
            Line::Node(code, node) => {
                self.network.insert(code, node);
            }
            Line::Directions(directions) => {
                self.directions = directions;
            }
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut moves;
        if self.gold {
            moves = 1;
            for start in self.network.keys() {
                if start.third() == 'A' {
                    moves = lcm(
                        moves,
                        self.moves(*start, |position| position.third() == 'Z')?,
                    );
                }
            }
        } else {
            moves = self.moves(START_POSITION, |position| position == END_POSITION)?;
        }
        println!(
            "[{}] Number of moves: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            moves
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(8, "Haunted Wasteland", Day8::new(gold)))
}
