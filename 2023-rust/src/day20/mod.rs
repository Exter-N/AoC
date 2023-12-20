use std::error::Error;

use aoc_common_rs::{
    cc::TwoCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
};

use self::circuit::{CircuitBuilder, Module};

mod circuit;

const RX: TwoCC = TwoCC::new('r', 'x');

#[derive(Debug)]
struct Day20 {
    gold: bool,
    export: bool,
    builder: CircuitBuilder,
}

impl Day20 {
    fn new(gold: bool, export: bool) -> Self {
        Self {
            gold,
            export,
            builder: CircuitBuilder::new(),
        }
    }
}

impl LineStreamHandler for Day20 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let ((module, module_id), outputs) = parse_full_string(
            line,
            separated_pair(
                alt((
                    map(tag("broadcaster"), |_| {
                        (Module::Broadcaster, Default::default())
                    }),
                    pair(
                        alt((
                            map(char('%'), |_| Module::new_flip_flop()),
                            map(char('&'), |_| Module::new_conjunction()),
                        )),
                        map(alpha1, TwoCC::from_lax),
                    ),
                )),
                tag(" -> "),
                separated_list1(tag(", "), map(alpha1, TwoCC::from_lax)),
            ),
        )?;
        self.builder.add_module(module_id, module, outputs);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            self.builder.add_module(RX, Module::new_counter(), vec![]);
        }
        let (mut circuit, mapping) = self.builder.build()?;
        if self.export {
            circuit.export(&mapping);
            return Ok(());
        }
        let broadcaster_index = mapping.index(&Default::default()).unwrap();
        if self.gold {
            let rx_index = mapping.index(&RX).unwrap();
            let triggers = circuit.triggers_before_low(broadcaster_index, rx_index);
            println!("[{}] Triggers before rx low: {}", GOLD_ANSI, triggers);
        } else {
            let mut total_low = 0;
            let mut total_high = 0;
            for _ in 0..1000 {
                let (low, high) = circuit.trigger(broadcaster_index);
                total_low += low;
                total_high += high;
            }
            println!("[-] Low pulses:        {}", total_low);
            println!("[-] High pulses:       {}", total_high);
            println!(
                "[{}] Product of pulses: {}",
                SILVER_ANSI,
                total_low * total_high
            );
        }
        Ok(())
    }
}

pub fn new(gold: bool, export: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(20, "Pulse Propagation", Day20::new(gold, export)).with_display_banner(!export))
}
