use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    digit::Digit,
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::character::complete::u32;

#[derive(Clone, Debug)]
struct Prng {
    state: u32,
}

impl Prng {
    const fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    const fn mix_and_prune(&mut self, value: u32) {
        self.state = (self.state ^ value) & 0xFFFFFF
    }

    const fn update(&mut self) {
        self.mix_and_prune(self.state << 6);
        self.mix_and_prune(self.state >> 5);
        self.mix_and_prune(self.state << 11);
    }

    const fn next_with_delta(&mut self) -> (Digit<10>, i8) {
        let previous = (self.state % 10) as i8;
        self.update();
        let new = (self.state % 10) as u8;
        (Digit::new(new).unwrap(), (new as i8) - previous)
    }
}

struct Day22 {
    sum: u64,
    bananas_by_sequence: HashMap<[i8; 4], u64>,
}

impl Day22 {
    fn new() -> Self {
        Self {
            sum: 0,
            bananas_by_sequence: HashMap::new(),
        }
    }
}

impl LineStreamHandler for Day22 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut prng = Prng::new(parse_full_string(line, u32)?);
        let mut deltas = [i8::MIN; 4];
        let mut seen_sequences = HashSet::new();
        for _ in 0usize..2000 {
            deltas.copy_within(1.., 0);
            let (bananas, delta) = prng.next_with_delta();
            deltas[3] = delta;
            if deltas.iter().all(|delta| *delta != i8::MIN) {
                if !seen_sequences.contains(&deltas) {
                    seen_sequences.insert(deltas);
                    let total_bananas = self.bananas_by_sequence.entry(deltas).or_insert(0);
                    *total_bananas += u64::from(bananas);
                }
            }
        }
        self.sum += prng.state as u64;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of 2000th secret numbers: {}",
            SILVER_ANSI, self.sum
        );
        let mut best_bananas = 0;
        let mut best_sequence = [i8::MIN; 4];
        for (sequence, bananas) in self.bananas_by_sequence {
            if bananas > best_bananas {
                best_bananas = bananas;
                best_sequence = sequence;
            }
        }
        println!(
            "[{}] Maximum bananas:              {}",
            GOLD_ANSI, best_bananas
        );
        println!("[-] Sequence for max. bananas:    {:?}", best_sequence);
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(22, "Monkey Market", Day22::new()))
}
