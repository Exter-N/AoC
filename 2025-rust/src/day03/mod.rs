use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    some_or_continue,
};

#[derive(Clone, Copy, Debug)]
struct PackedBuffer {
    value: u64,
    mask: u64,
    last_offset: usize,
    divisor: u64,
}

impl PackedBuffer {
    fn new(len: usize) -> Self {
        let bit_len = len * 5;
        Self {
            value: 0,
            mask: (1 << bit_len) - 1,
            last_offset: bit_len - 5,
            divisor: 10u64.pow((12 - len) as u32),
        }
    }

    fn insert(&mut self, digit: u32) {
        const MSB_MASK: u64 = 0x842108421084210;
        const LSB_MASK: u64 = 0x84210842108421;
        let insert = (digit as u64) << self.last_offset;
        let shifted = (self.value >> 5) | insert | (MSB_MASK & self.mask);
        let mut position = ((shifted - self.value - LSB_MASK) & MSB_MASK).trailing_zeros();
        position -= position % 5;
        let fixed_mask = (1 << position) - 1;
        if fixed_mask >= self.mask {
            return;
        }
        let moving_mask = self.mask - ((1 << (position + 5)) - 1);
        self.value = (self.value & fixed_mask) | ((self.value & moving_mask) >> 5) | insert;
    }

    fn decode(&self) -> u64 {
        let mut value = self.value;
        value = ((value & 0x7c1f07c1f07c1f) * 10) + ((value & 0xf83e0f83e0f83e0) >> 5);
        value = ((value & 0x3ff003ff003ff) * 100) + ((value & 0xffc00ffc00ffc00) >> 10);
        value = ((value & 0xfffff) * 100000000)
            + (((value & 0xfffff00000) >> 20) * 10000)
            + ((value & 0xfffff0000000000) >> 40);
        value / self.divisor
    }
}

struct Day3 {
    total: u64,
    batteries_per_bank: usize,
}

impl Day3 {
    fn new(gold: bool) -> Self {
        Self {
            total: 0,
            batteries_per_bank: if gold { 12 } else { 2 },
        }
    }
}

impl LineStreamHandler for Day3 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut buffer_packed = PackedBuffer::new(self.batteries_per_bank);
        for ch in line.chars() {
            let digit = some_or_continue!(ch.to_digit(10));
            buffer_packed.insert(digit);
        }
        self.total += buffer_packed.decode();

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total output joltage: {}",
            if self.batteries_per_bank > 10 {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.total
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(3, "Lobby", Day3::new(gold)))
}
