use std::error::Error;

use nom::combinator::opt;

use crate::ord::binary_search;

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

mod packet;

use packet::{divider, packet, Packet};

#[derive(Default)]
struct Day13 {
    well_ordered_index_sum: usize,
    packets: Vec<Packet>,
}

impl Day13 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day13 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        if let Some(packet) = parse_full_string(line, opt(packet))? {
            self.packets.push(packet);
            let packets_so_far = self.packets.len();
            if 0 == (packets_so_far % 2) {
                if self.packets[packets_so_far - 2] < self.packets[packets_so_far - 1] {
                    self.well_ordered_index_sum += packets_so_far / 2;
                }
            }
        }

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        self.packets.sort_unstable();

        println!(
            "[{}] Well-ordered index sum: {}",
            SILVER_ANSI, self.well_ordered_index_sum
        );

        let (_, pos2) = binary_search(&self.packets, &divider(2));
        let (_, pos6) = binary_search(&self.packets, &divider(6));

        println!(
            "[{}] Decoder key:            {}",
            GOLD_ANSI,
            (pos2 + 1) * (pos6 + 2)
        );

        Ok(())
    }
}

pub fn new() -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((13, "Distress Signal", Box::new(Day13::new())))
}
