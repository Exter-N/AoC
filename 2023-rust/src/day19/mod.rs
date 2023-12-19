use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    error::Error,
    ops::RangeInclusive,
};

use aoc_common_rs::{
    cc::ThreeCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandlerOnce},
};
use nom::{
    branch::alt,
    character::complete::{alpha1, char, one_of, u16},
    combinator::{map_res, opt, value},
    multi::fold_many1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
};

#[derive(Clone, Copy)]
enum PartRating {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for PartRating {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err("invalid part rating".into()),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn new() -> Self {
        Default::default()
    }
    fn rating(self, rating: PartRating) -> u16 {
        match rating {
            PartRating::X => self.x,
            PartRating::M => self.m,
            PartRating::A => self.a,
            PartRating::S => self.s,
        }
    }
    fn with_rating(self, rating: PartRating, value: u16) -> Self {
        match rating {
            PartRating::X => Self { x: value, ..self },
            PartRating::M => Self { m: value, ..self },
            PartRating::A => Self { a: value, ..self },
            PartRating::S => Self { s: value, ..self },
        }
    }
    fn total_rating(self) -> u16 {
        self.x + self.m + self.a + self.s
    }
}

fn partition_range(
    range: RangeInclusive<u16>,
    ordering: Ordering,
    threshold: u16,
) -> (Option<RangeInclusive<u16>>, Option<RangeInclusive<u16>>) {
    match ordering {
        Ordering::Equal => unimplemented!(),
        Ordering::Less => {
            if *range.end() < threshold {
                (Some(range), None)
            } else if *range.start() >= threshold {
                (None, Some(range))
            } else {
                (
                    Some(*range.start()..=(threshold - 1)),
                    Some(threshold..=*range.end()),
                )
            }
        }
        Ordering::Greater => {
            if *range.start() > threshold {
                (Some(range), None)
            } else if *range.end() <= threshold {
                (None, Some(range))
            } else {
                (
                    Some((threshold + 1)..=*range.end()),
                    Some(*range.start()..=threshold),
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PartClass {
    x: RangeInclusive<u16>,
    m: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
}

impl PartClass {
    fn new(range: RangeInclusive<u16>) -> Self {
        Self {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range,
        }
    }
    fn rating(&self, rating: PartRating) -> RangeInclusive<u16> {
        match rating {
            PartRating::X => self.x.clone(),
            PartRating::M => self.m.clone(),
            PartRating::A => self.a.clone(),
            PartRating::S => self.s.clone(),
        }
    }
    fn with_rating(&self, rating: PartRating, value: RangeInclusive<u16>) -> Self {
        match rating {
            PartRating::X => Self {
                x: value,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartRating::M => Self {
                x: self.x.clone(),
                m: value,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartRating::A => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: value,
                s: self.s.clone(),
            },
            PartRating::S => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: value,
            },
        }
    }
    fn partition(
        self,
        rating: PartRating,
        ordering: Ordering,
        threshold: u16,
    ) -> (Option<Self>, Option<Self>) {
        let (matching, non_matching) = partition_range(self.rating(rating), ordering, threshold);
        (
            matching.map(|rg| self.with_rating(rating, rg)),
            non_matching.map(|rg| self.with_rating(rating, rg)),
        )
    }
    fn count(&self) -> u64 {
        (self.x.end() + 1 - self.x.start()) as u64
            * (self.m.end() + 1 - self.m.start()) as u64
            * (self.a.end() + 1 - self.a.start()) as u64
            * (self.s.end() + 1 - self.s.start()) as u64
    }
}

fn parse_workflow_id(value: &str) -> Result<ThreeCC, Box<dyn Error>> {
    if value.len() <= 3 {
        Ok(ThreeCC::from_lax(value))
    } else {
        Err("workflow ID cannot be longer than 3 chars".into())
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Accept,
    Reject,
    Jump(ThreeCC),
}

impl TryFrom<&str> for Action {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "A" {
            Ok(Self::Accept)
        } else if value == "R" {
            Ok(Self::Reject)
        } else {
            Ok(Self::Jump(parse_workflow_id(value)?))
        }
    }
}

struct Workflow {
    rules: Vec<(PartRating, Ordering, u16, Action)>,
    default: Action,
}

impl Workflow {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
            default: Action::Reject,
        }
    }
    fn apply(&self, part: Part) -> Action {
        for (rating, ordering, threshold, action) in self.rules.iter() {
            if part.rating(*rating).cmp(threshold) == *ordering {
                return *action;
            }
        }
        self.default
    }
    fn apply_class(
        &self,
        mut part_class: PartClass,
        next: &mut VecDeque<(PartClass, ThreeCC)>,
    ) -> u64 {
        let mut accepted_count = 0;
        for (rating, ordering, threshold, action) in self.rules.iter() {
            let (matching, non_matching) = part_class.partition(*rating, *ordering, *threshold);
            if let Some(class) = matching {
                match action {
                    Action::Accept => {
                        accepted_count += class.count();
                    }
                    Action::Reject => {}
                    Action::Jump(workflow_id) => {
                        next.push_back((class, *workflow_id));
                    }
                }
            }
            if let Some(class) = non_matching {
                part_class = class;
            } else {
                return accepted_count;
            }
        }
        match self.default {
            Action::Accept => {
                accepted_count += part_class.count();
            }
            Action::Reject => {}
            Action::Jump(workflow_id) => {
                next.push_back((part_class, workflow_id));
            }
        }
        accepted_count
    }
}

struct Day19 {
    gold: bool,
    workflows: HashMap<ThreeCC, Workflow>,
    total_rating: u32,
}

impl Day19 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            workflows: HashMap::new(),
            total_rating: 0,
        }
    }
}

struct Day19Workflows(Day19);

impl LineStreamHandlerOnce for Day19Workflows {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(if self.0.gold {
                Box::new(Day19Satisfiability(self.0))
            } else {
                Box::new(Day19Parts(self.0))
            });
        }

        let (id, workflow) = parse_full_string(
            line,
            pair(
                map_res(alpha1, parse_workflow_id),
                delimited(
                    char('{'),
                    fold_many1(
                        terminated(
                            pair(
                                opt(terminated(
                                    tuple((
                                        map_res(one_of("xmas"), PartRating::try_from),
                                        alt((
                                            value(Ordering::Less, char('<')),
                                            value(Ordering::Greater, char('>')),
                                        )),
                                        u16,
                                    )),
                                    char(':'),
                                )),
                                map_res(alpha1, Action::try_from),
                            ),
                            opt(char(',')),
                        ),
                        || Workflow::new(),
                        |mut workflow, (condition, action)| {
                            if let Some((rating, ordering, threshold)) = condition {
                                workflow.rules.push((rating, ordering, threshold, action));
                            } else {
                                workflow.default = action;
                            }
                            workflow
                        },
                    ),
                    char('}'),
                ),
            ),
        )?;
        self.0.workflows.insert(id, workflow);

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

struct Day19Parts(Day19);

impl LineStreamHandlerOnce for Day19Parts {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let part = parse_full_string(
            line,
            delimited(
                char('{'),
                fold_many1(
                    terminated(
                        separated_pair(
                            map_res(one_of("xmas"), PartRating::try_from),
                            char('='),
                            u16,
                        ),
                        opt(char(',')),
                    ),
                    || Part::new(),
                    |part, (rating, value)| part.with_rating(rating, value),
                ),
                char('}'),
            ),
        )?;
        let mut action = Action::Jump(ThreeCC::new('i', 'n', '\0'));
        while let Action::Jump(workflow) = action {
            action = self.0.workflows[&workflow].apply(part);
        }
        if matches!(action, Action::Accept) {
            self.0.total_rating += part.total_rating() as u32;
        }

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total accepted part rating: {}",
            SILVER_ANSI, self.0.total_rating
        );
        Ok(())
    }
}

struct Day19Satisfiability(Day19);

impl LineStreamHandlerOnce for Day19Satisfiability {
    fn update(
        self: Box<Self>,
        _line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut accepted_count = 0;
        let mut next = VecDeque::new();
        next.push_back((PartClass::new(1..=4000), ThreeCC::new('i', 'n', '\0')));
        while let Some((part_class, workflow_id)) = next.pop_front() {
            accepted_count += self.0.workflows[&workflow_id].apply_class(part_class, &mut next);
        }
        println!(
            "[{}] Distinct accepted parts: {}",
            GOLD_ANSI, accepted_count
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(
        19,
        "Aplenty",
        Day19Workflows(Day19::new(gold)),
    ))
}
