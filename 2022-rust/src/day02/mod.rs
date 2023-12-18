use std::error::Error;

use nom::character::complete::{anychar, char};
use nom::sequence::separated_pair;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

mod rps;

use rps::{Gesture, Outcome};

#[derive(Default)]
struct Day2 {
    second_char_is_outcome: bool,
    total_score: u32,
}

impl Day2 {
    fn new(second_char_is_outcome: bool) -> Self {
        Self {
            second_char_is_outcome,
            ..Default::default()
        }
    }
}

impl LineStreamHandler for Day2 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (theirs_c, second_c) =
            parse_full_string(line, separated_pair(anychar, char(' '), anychar))?;
        let theirs = Gesture::try_from(theirs_c)?;
        let ours;
        let outcome;
        if self.second_char_is_outcome {
            outcome = Outcome::try_from(second_c)?;
            ours = outcome.what_must_we_play(theirs);
        } else {
            ours = Gesture::try_from(second_c)?;
            outcome = ours.play_against(theirs);
        }
        self.total_score += (1 + ours as u32) + (3 * outcome as u32);

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total score: {}",
            if self.second_char_is_outcome {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.total_score
        );

        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(2, "Rock Paper Scissors", Day2::new(gold)))
}
