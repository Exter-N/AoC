use std::{collections::HashMap, error::Error, hash::Hash, ops::Deref};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, multispace1, u32},
    combinator::{map, value},
    multi::{many1, many1_count, separated_list1},
    sequence::separated_pair,
};

#[derive(Clone, Copy, Debug)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Default, Debug)]
struct SpringStates<'a> {
    states: &'a [SpringState],
    num_operational: usize,
    num_damaged: usize,
    num_unknown: usize,
}

impl<'a> SpringStates<'a> {
    fn new(states: &'a [SpringState]) -> Self {
        let mut num_operational = 0;
        let mut num_damaged = 0;
        let mut num_unknown = 0;
        for state in states {
            match state {
                SpringState::Operational => {
                    num_operational += 1;
                }
                SpringState::Damaged => {
                    num_damaged += 1;
                }
                SpringState::Unknown => {
                    num_unknown += 1;
                }
            }
        }
        Self {
            states,
            num_operational,
            num_damaged,
            num_unknown,
        }
    }
    fn without_front(&self) -> Self {
        Self {
            states: &self.states[1..],
            num_operational: self.num_operational
                - if matches!(self.states[0], SpringState::Operational) {
                    1
                } else {
                    0
                },
            num_damaged: self.num_damaged
                - if matches!(self.states[0], SpringState::Damaged) {
                    1
                } else {
                    0
                },
            num_unknown: self.num_unknown
                - if matches!(self.states[0], SpringState::Unknown) {
                    1
                } else {
                    0
                },
        }
    }
    fn try_match_at_start(&self, run: usize) -> Option<SpringStates<'a>> {
        if self.num_damaged + self.num_unknown < run {
            return None;
        }
        let mut num_damaged = 0;
        let mut num_unknown = 0;
        for i in 0..run {
            match self.states[i] {
                SpringState::Operational => {
                    return None;
                }
                SpringState::Damaged => {
                    num_damaged += 1;
                }
                SpringState::Unknown => {
                    num_unknown += 1;
                }
            }
        }
        if self.states.len() > run {
            let num_operational = match self.states[run] {
                SpringState::Operational => 1,
                SpringState::Damaged => {
                    return None;
                }
                SpringState::Unknown => {
                    num_unknown += 1;
                    0
                }
            };
            Some(Self {
                states: &self.states[(run + 1)..],
                num_operational: self.num_operational - num_operational,
                num_damaged: self.num_damaged - num_damaged,
                num_unknown: self.num_unknown - num_unknown,
            })
        } else {
            Some(Default::default())
        }
    }
    fn try_match_at_end(&self, run: usize) -> Option<SpringStates<'a>> {
        if self.num_damaged + self.num_unknown < run {
            return None;
        }
        let num_states = self.states.len();
        let mut num_damaged = 0;
        let mut num_unknown = 0;
        for i in 0..run {
            match self.states[num_states - 1 - i] {
                SpringState::Operational => {
                    return None;
                }
                SpringState::Damaged => {
                    num_damaged += 1;
                }
                SpringState::Unknown => {
                    num_unknown += 1;
                }
            }
        }
        if self.states.len() > run {
            let num_operational = match self.states[num_states - 1 - run] {
                SpringState::Operational => 1,
                SpringState::Damaged => {
                    return None;
                }
                SpringState::Unknown => {
                    num_unknown += 1;
                    0
                }
            };
            Some(Self {
                states: &self.states[..(num_states - 1 - run)],
                num_operational: self.num_operational - num_operational,
                num_damaged: self.num_damaged - num_damaged,
                num_unknown: self.num_unknown - num_unknown,
            })
        } else {
            Some(Default::default())
        }
    }
}

impl Deref for SpringStates<'_> {
    type Target = [SpringState];

    fn deref(&self) -> &Self::Target {
        self.states
    }
}

fn trim_operational<'a>(mut states: SpringStates<'a>) -> SpringStates<'a> {
    if matches!(states.first(), Some(SpringState::Operational)) {
        states.states = &states.states[1..];
        states.num_operational -= 1;
    }
    if matches!(states.last(), Some(SpringState::Operational)) {
        states.states = &states.states[..(states.len() - 1)];
        states.num_operational -= 1;
    }
    states
}

fn trim_fixed_damaged<'a, 'b>(
    mut states: SpringStates<'a>,
    mut damaged_runs: &'b [usize],
) -> Result<(SpringStates<'a>, &'b [usize]), &'static str> {
    while let Some(run) = damaged_runs.first() {
        if !matches!(states.first(), Some(SpringState::Damaged)) {
            break;
        }
        let num_states = states.len();
        if num_states < *run {
            return Err("not enough states".into());
        }
        match states.try_match_at_start(*run) {
            Some(rest) => {
                states = rest;
                damaged_runs = &damaged_runs[1..];
            }
            None => {
                return Err("inconsistent states".into());
            }
        }
    }
    while let Some(run) = damaged_runs.last() {
        if !matches!(states.last(), Some(SpringState::Damaged)) {
            break;
        }
        let num_states = states.len();
        if num_states < *run {
            return Err("not enough states".into());
        }
        match states.try_match_at_end(*run) {
            Some(rest) => {
                states = rest;
                damaged_runs = &damaged_runs[..(damaged_runs.len() - 1)];
            }
            None => {
                return Err("inconsistent states".into());
            }
        }
    }
    Ok((states, damaged_runs))
}

struct ArrangementsKey<'a, 'b>(&'a [SpringState], &'b [usize]);

impl PartialEq for ArrangementsKey<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr_range() == other.0.as_ptr_range()
            && self.1.as_ptr_range() == other.1.as_ptr_range()
    }
}

impl Eq for ArrangementsKey<'_, '_> {}

impl Hash for ArrangementsKey<'_, '_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ptr_range().hash(state);
        self.1.as_ptr_range().hash(state);
    }
}

fn arrangements<'a, 'b>(
    mut states: SpringStates<'a>,
    mut damaged_runs: &'b [usize],
    cache: &mut HashMap<ArrangementsKey<'a, 'b>, Result<usize, &'static str>>,
) -> Result<usize, &'static str> {
    let sum_of_runs: usize = damaged_runs.iter().sum();
    if states.num_damaged + states.num_unknown < sum_of_runs {
        return Err("not enough damaged/unknowns".into());
    }
    if states.num_damaged > sum_of_runs {
        return Err("too many damaged".into());
    }
    if damaged_runs.len() == 0 {
        return Ok(1);
    }
    if states.num_operational + states.num_unknown < damaged_runs.len() - 1 {
        return Err("not enough operational/unknowns".into());
    }
    if states.len() < sum_of_runs + damaged_runs.len() - 1 {
        return Err("not enough states".into());
    }
    states = trim_operational(states);
    (states, damaged_runs) = trim_fixed_damaged(states, damaged_runs)?;
    if states.len() == 0 {
        return if damaged_runs.len() == 0 {
            Ok(1)
        } else {
            Err("not enough states".into())
        };
    }
    if let Some(run) = damaged_runs.first() {
        Ok(match states.try_match_at_start(*run) {
            Some(rest) => memoized_arrangements(rest, &damaged_runs[1..], cache).unwrap_or(0),
            None => 0,
        } + memoized_arrangements(states.without_front(), damaged_runs, cache).unwrap_or(0))
    } else {
        if states
            .iter()
            .any(|state| matches!(state, SpringState::Damaged))
        {
            Err("inconsistent states".into())
        } else {
            Ok(1)
        }
    }
}

fn memoized_arrangements<'a, 'b>(
    states: SpringStates<'a>,
    damaged_runs: &'b [usize],
    cache: &mut HashMap<ArrangementsKey<'a, 'b>, Result<usize, &'static str>>,
) -> Result<usize, &'static str> {
    let key = ArrangementsKey(states.states, damaged_runs);
    if let Some(result) = cache.get(&key) {
        *result
    } else {
        let result = arrangements(states, damaged_runs, cache);
        cache.insert(key, result);
        result
    }
}

struct Day12 {
    gold: bool,
    sum_of_arrangements: usize,
}

impl Day12 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            sum_of_arrangements: 0,
        }
    }
}

impl LineStreamHandler for Day12 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (states, damaged_runs) = parse_full_string(
            line,
            separated_pair(
                many1(alt((
                    value(SpringState::Operational, many1_count(char('.'))),
                    value(SpringState::Damaged, char('#')),
                    value(SpringState::Unknown, char('?')),
                ))),
                multispace1,
                separated_list1(char(','), map(u32, |n| n as usize)),
            ),
        )?;
        if self.gold {
            let mut states_extended = (0..5)
                .cartesian_product(states.iter().chain(vec![SpringState::Unknown].iter()))
                .map(|(_, state)| *state)
                .collect::<Vec<_>>();
            states_extended.pop();
            let damaged_runs_extended = (0..5)
                .cartesian_product(damaged_runs.iter())
                .map(|(_, run)| *run)
                .collect::<Vec<_>>();
            self.sum_of_arrangements += arrangements(
                SpringStates::new(&states_extended[..]),
                &damaged_runs_extended[..],
                &mut HashMap::new(),
            )?;
        } else {
            self.sum_of_arrangements += arrangements(
                SpringStates::new(&states[..]),
                &damaged_runs[..],
                &mut HashMap::new(),
            )?;
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of arrangements: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.sum_of_arrangements
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(12, "Hot Springs", Day12::new(gold)))
}
