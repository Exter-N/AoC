use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, u32};
use nom::combinator::{map, opt};
use nom::error::Error as NomError;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair, tuple};

use aoc_common_rs::{
    cc::TwoCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    ord::ProximityMap,
};

mod net;
mod valve;

use net::ValveNetwork;
use valve::Valve;

#[derive(Debug, Default)]
struct Day16 {
    valves: ValveNetwork,
}

impl Day16 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day16 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let two_cc = move || map(pair(anychar::<&str, NomError<&str>>, anychar), TwoCC::from);
        let (id, valve) = parse_full_string(
            line,
            preceded(
                tag("Valve "),
                separated_pair(
                    two_cc(),
                    tag(" has flow rate="),
                    map(
                        separated_pair(
                            u32,
                            tuple((
                                tag("; tunnel"),
                                opt(char('s')),
                                tag(" lead"),
                                opt(char('s')),
                                tag(" to valve"),
                                opt(char('s')),
                                char(' '),
                            )),
                            map(separated_list1(tag(", "), two_cc()), |neighbors_v| {
                                let mut neighbors: ProximityMap<TwoCC, u16> = ProximityMap::new();
                                for neighbor in neighbors_v {
                                    neighbors.insert(neighbor, 1);
                                }

                                neighbors
                            }),
                        ),
                        |(flow_rate, neighbors)| Valve::new(flow_rate, neighbors),
                    ),
                ),
            ),
        )?;

        self.valves.insert(id, valve);

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.valves.add_virtual_starter()?;
        self.valves.prepare()?;
        let solo = self.valves.max_pressure_release(30, 1)?;
        // With sample input: 1651 (DD 28*20, BB 25*13, JJ 21*21, HH 13*22, EE  9* 3, CC  6* 2)
        println!(
            "[{}] Solo max pressure release in 30': {} {:?}",
            SILVER_ANSI, solo.0, solo.1
        );
        let duo = self.valves.max_pressure_release(26, 2)?;
        // With sample input: 1707 (DD 24*20, JJ 23*21, HH 19*22, BB 19*21, CC 17* 2, EE 15* 3)
        println!(
            "[{}] Duo max pressure release in 26':  {} {:?}",
            GOLD_ANSI, duo.0, duo.1
        );

        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(16, "Proboscidea Volcanium", Day16::new()))
}
