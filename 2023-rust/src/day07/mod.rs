use std::{cmp::Ordering, error::Error};

use nom::{
    character::complete::{char, one_of, u32},
    combinator::{map, map_res},
    sequence::separated_pair,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    unwrap_either,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn jokerize(self, condition: bool) -> Self {
        if condition && self == Self::Jack {
            Self::Joker
        } else {
            self
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Box<dyn Error>> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("value".into()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ClassifiedHand {
    FiveOfAKind {
        five: Card,
    },
    FourOfAKind {
        four: Card,
        lone: Card,
    },
    FullHouse {
        three: Card,
        pair: Card,
    },
    ThreeOfAKind {
        three: Card,
        lone0: Card,
        lone1: Card,
    },
    TwoPair {
        pair0: Card,
        pair1: Card,
        lone: Card,
    },
    OnePair {
        pair: Card,
        lone0: Card,
        lone1: Card,
        lone2: Card,
    },
    HighCard {
        lone0: Card,
        lone1: Card,
        lone2: Card,
        lone3: Card,
        lone4: Card,
    },
}

impl ClassifiedHand {
    fn hand_type(self) -> HandType {
        match self {
            Self::FiveOfAKind { five: _ } => HandType::FiveOfAKind,
            Self::FourOfAKind {
                four: Card::Joker,
                lone: _,
            } => HandType::FiveOfAKind,
            Self::FourOfAKind {
                four: _,
                lone: Card::Joker,
            } => HandType::FiveOfAKind,
            Self::FourOfAKind { four: _, lone: _ } => HandType::FourOfAKind,
            Self::FullHouse {
                three: Card::Joker,
                pair: _,
            } => HandType::FiveOfAKind,
            Self::FullHouse {
                three: _,
                pair: Card::Joker,
            } => HandType::FiveOfAKind,
            Self::FullHouse { three: _, pair: _ } => HandType::FullHouse,
            Self::ThreeOfAKind {
                three: Card::Joker,
                lone0: _,
                lone1: _,
            } => HandType::FourOfAKind,
            Self::ThreeOfAKind {
                three: _,
                lone0: _,
                lone1: Card::Joker,
            } => HandType::FourOfAKind,
            Self::ThreeOfAKind {
                three: _,
                lone0: _,
                lone1: _,
            } => HandType::ThreeOfAKind,
            Self::TwoPair {
                pair0: _,
                pair1: Card::Joker,
                lone: _,
            } => HandType::FourOfAKind,
            Self::TwoPair {
                pair0: _,
                pair1: _,
                lone: Card::Joker,
            } => HandType::FullHouse,
            Self::TwoPair {
                pair0: _,
                pair1: _,
                lone: _,
            } => HandType::TwoPair,
            Self::OnePair {
                pair: Card::Joker,
                lone0: _,
                lone1: _,
                lone2: _,
            } => HandType::ThreeOfAKind,
            Self::OnePair {
                pair: _,
                lone0: _,
                lone1: _,
                lone2: Card::Joker,
            } => HandType::ThreeOfAKind,
            Self::OnePair {
                pair: _,
                lone0: _,
                lone1: _,
                lone2: _,
            } => HandType::OnePair,
            Self::HighCard {
                lone0: _,
                lone1: _,
                lone2: _,
                lone3: _,
                lone4: Card::Joker,
            } => HandType::OnePair,
            Self::HighCard {
                lone0: _,
                lone1: _,
                lone2: _,
                lone3: _,
                lone4: _,
            } => HandType::HighCard,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
struct Hand([Card; 5]);

impl Hand {
    fn new(first: Card, second: Card, third: Card, fourth: Card, fifth: Card) -> Self {
        Self([first, second, third, fourth, fifth])
    }

    fn classify(&self) -> ClassifiedHand {
        let mut hand = self.0;
        hand.sort();
        match (
            hand[0] == hand[1],
            hand[1] == hand[2],
            hand[2] == hand[3],
            hand[3] == hand[4],
        ) {
            (true, true, true, true) => ClassifiedHand::FiveOfAKind { five: hand[0] },
            (true, true, true, false) => ClassifiedHand::FourOfAKind {
                four: hand[0],
                lone: hand[4],
            },
            (true, true, false, true) => ClassifiedHand::FullHouse {
                three: hand[0],
                pair: hand[3],
            },
            (true, true, false, false) => ClassifiedHand::ThreeOfAKind {
                three: hand[0],
                lone0: hand[3],
                lone1: hand[4],
            },
            (true, false, true, true) => ClassifiedHand::FullHouse {
                three: hand[2],
                pair: hand[0],
            },
            (true, false, true, false) => ClassifiedHand::TwoPair {
                pair0: hand[0],
                pair1: hand[2],
                lone: hand[4],
            },
            (true, false, false, true) => ClassifiedHand::TwoPair {
                pair0: hand[0],
                pair1: hand[3],
                lone: hand[2],
            },
            (true, false, false, false) => ClassifiedHand::OnePair {
                pair: hand[0],
                lone0: hand[2],
                lone1: hand[3],
                lone2: hand[4],
            },
            (false, true, true, true) => ClassifiedHand::FourOfAKind {
                four: hand[1],
                lone: hand[0],
            },
            (false, true, true, false) => ClassifiedHand::ThreeOfAKind {
                three: hand[1],
                lone0: hand[0],
                lone1: hand[4],
            },
            (false, true, false, true) => ClassifiedHand::TwoPair {
                pair0: hand[1],
                pair1: hand[3],
                lone: hand[0],
            },
            (false, true, false, false) => ClassifiedHand::OnePair {
                pair: hand[1],
                lone0: hand[0],
                lone1: hand[3],
                lone2: hand[4],
            },
            (false, false, true, true) => ClassifiedHand::ThreeOfAKind {
                three: hand[2],
                lone0: hand[0],
                lone1: hand[1],
            },
            (false, false, true, false) => ClassifiedHand::OnePair {
                pair: hand[2],
                lone0: hand[0],
                lone1: hand[1],
                lone2: hand[4],
            },
            (false, false, false, true) => ClassifiedHand::OnePair {
                pair: hand[3],
                lone0: hand[0],
                lone1: hand[1],
                lone2: hand[2],
            },
            (false, false, false, false) => ClassifiedHand::HighCard {
                lone0: hand[0],
                lone1: hand[1],
                lone2: hand[2],
                lone3: hand[3],
                lone4: hand[4],
            },
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self
            .classify()
            .hand_type()
            .cmp(&other.classify().hand_type())
        {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.0.cmp(&other.0)
    }
}

struct Day7 {
    gold: bool,
    hands: Vec<(Hand, u32)>,
}

impl Day7 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            hands: Vec::new(),
        }
    }
}

impl LineStreamHandler for Day7 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (hand, bid) = parse_full_string(
            line,
            separated_pair(
                map(
                    (
                        map_res(one_of("23456789AJKQT"), |card| {
                            Card::try_from(card).map(|card| card.jokerize(self.gold))
                        }),
                        map_res(one_of("23456789AJKQT"), |card| {
                            Card::try_from(card).map(|card| card.jokerize(self.gold))
                        }),
                        map_res(one_of("23456789AJKQT"), |card| {
                            Card::try_from(card).map(|card| card.jokerize(self.gold))
                        }),
                        map_res(one_of("23456789AJKQT"), |card| {
                            Card::try_from(card).map(|card| card.jokerize(self.gold))
                        }),
                        map_res(one_of("23456789AJKQT"), |card| {
                            Card::try_from(card).map(|card| card.jokerize(self.gold))
                        }),
                    ),
                    |(first, second, third, fourth, fifth)| {
                        Hand::new(first, second, third, fourth, fifth)
                    },
                ),
                char(' '),
                u32,
            ),
        )?;
        let i = unwrap_either(self.hands.binary_search_by_key(&hand, |(hand, _)| *hand));
        self.hands.insert(i, (hand, bid));
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut score = 0;
        let num_hands = self.hands.len();
        for ((_, bid), rank) in self.hands.into_iter().zip((0..num_hands).rev()) {
            score += bid * (rank + 1) as u32;
        }
        println!(
            "[{}] Total winnings: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            score
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(7, "Camel Cards", Day7::new(gold)))
}
