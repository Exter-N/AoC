use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

mod item_set;

use item_set::ItemSet;

struct Day3 {
    error_sum: u32,
    badge_sum: u32,
    current_items: ItemSet,
    pos_in_group: u32,
}

impl Day3 {
    fn new() -> Self {
        Self {
            error_sum: 0,
            badge_sum: 0,
            current_items: ItemSet::empty(),
            pos_in_group: 0,
        }
    }
}

impl LineStreamHandler for Day3 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if 0 != (line.len() & 1) {
            return Err(Box::from("imbalanced halfs"));
        }
        let (left_half, right_half) = line.split_at(line.len() >> 1);
        let left_set = ItemSet::try_from(left_half)?;
        let right_set = ItemSet::try_from(right_half)?;
        self.error_sum += (left_set & right_set).singleton_priority()?;
        let full_set = left_set | right_set;
        if 0 == self.pos_in_group {
            self.current_items = full_set;
        } else {
            self.current_items &= full_set;
        }
        if 2 == self.pos_in_group {
            self.badge_sum += self.current_items.singleton_priority()?;
        }
        self.pos_in_group = (self.pos_in_group + 1) % 3;

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!("[{}] Error sum: {}", SILVER_ANSI, self.error_sum);
        println!("[{}] Badge sum: {}", GOLD_ANSI, self.badge_sum);

        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(3, "Rucksack Reorganization", Day3::new()))
}
