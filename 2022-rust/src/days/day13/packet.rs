use std::cmp::Ordering;
use std::rc::Rc;

use itertools::{EitherOrBoth, Itertools};

use nom::branch::alt;
use nom::character::complete::{char, u32};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Clone, Debug)]
pub enum Packet {
    Integer(u32),
    List(Rc<Vec<Packet>>),
}

impl Packet {
    fn from_list(list: Vec<Packet>) -> Self {
        Self::List(Rc::new(list))
    }
    fn to_list(&self) -> Rc<Vec<Packet>> {
        match self {
            Packet::Integer(_) => Rc::new(vec![self.clone()]),
            Packet::List(list) => list.clone(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Packet::Integer(self_num) = self {
            if let Packet::Integer(other_num) = other {
                return self_num.cmp(other_num);
            }
        }

        for subs in self.to_list().iter().zip_longest(other.to_list().iter()) {
            match subs {
                EitherOrBoth::Both(self_sub, other_sub) => {
                    let sub_cmp = self_sub.cmp(other_sub);
                    if sub_cmp != Ordering::Equal {
                        return sub_cmp;
                    }
                }
                EitherOrBoth::Left(_) => {
                    return Ordering::Greater;
                }
                EitherOrBoth::Right(_) => {
                    return Ordering::Less;
                }
            }
        }

        Ordering::Equal
    }
}

pub fn packet<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, Packet, E> {
    alt((
        map(
            delimited(char('['), separated_list0(char(','), packet), char(']')),
            |list| Packet::List(Rc::new(list)),
        ),
        map(u32, |num| Packet::Integer(num)),
    ))(s)
}

pub fn divider(code: u32) -> Packet {
    Packet::from_list(vec![Packet::from_list(vec![Packet::Integer(code)])])
}
