use std::{collections::HashMap, error::Error};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    digit::Digit,
    line_stream::LineStreamHandler,
    ok_or_continue,
};
use pads::{next_cost_matrix, DpadKey, KeyPressCost, NumpadKey};

mod pads;

struct Day21 {
    gold: bool,
    cost_matrix: HashMap<(NumpadKey, NumpadKey), KeyPressCost>,
    sum_of_complexities: usize,
}

impl Day21 {
    fn new(gold: bool) -> Self {
        let mut cost_matrix = DpadKey::initial_cost_matrix();
        for _ in 0usize..(if gold { 25 } else { 2 }) {
            cost_matrix = next_cost_matrix::<DpadKey>(&cost_matrix);
        }
        let cost_matrix = next_cost_matrix::<NumpadKey>(&cost_matrix);
        Self {
            gold,
            cost_matrix,
            sum_of_complexities: 0,
        }
    }
}

impl LineStreamHandler for Day21 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut value = 0usize;
        let mut cost = KeyPressCost::zero();
        let mut last_key = NumpadKey::A;
        for ch in line.chars() {
            let key = ok_or_continue!(NumpadKey::try_from(ch));
            if let NumpadKey::Digit(digit) = key {
                value = Digit::append_lowest(value, digit);
            }
            let edge_cost = self.cost_matrix.get(&(last_key, key)).unwrap();
            cost += edge_cost.clone();
            last_key = key;
        }
        self.sum_of_complexities += value * cost.cost;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of complexities: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.sum_of_complexities
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(21, "Keypad Conundrum", Day21::new(gold)))
}
