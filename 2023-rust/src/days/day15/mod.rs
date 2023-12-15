use std::{error::Error, mem::take};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

fn key_update(key: &mut u64, current: char) {
    *key = (*key << 8) | current as u64
}

fn hash_update(hash: &mut u8, current: char) {
    *hash = hash.wrapping_add(current as u8).wrapping_mul(17)
}

#[derive(Default, Debug)]
enum InstructionOp {
    #[default]
    Incomplete,
    Set(usize),
    Remove,
}

#[derive(Default, Debug)]
struct Instruction {
    key: u64,
    op: InstructionOp,
    bucket: u8,
    hash: u8,
}

impl Instruction {
    fn update(&mut self, ch: char) -> Result<(), Box<dyn Error>> {
        hash_update(&mut self.hash, ch);
        match self.op {
            InstructionOp::Incomplete => match ch {
                '-' => {
                    self.op = InstructionOp::Remove;
                    Ok(())
                }
                '=' => {
                    self.op = InstructionOp::Set(0);
                    Ok(())
                }
                _ => {
                    key_update(&mut self.key, ch);
                    hash_update(&mut self.bucket, ch);
                    Ok(())
                }
            },
            InstructionOp::Set(value) => {
                if let Some(digit) = ch.to_digit(10) {
                    self.op = InstructionOp::Set((value * 10) + digit as usize);
                    Ok(())
                } else {
                    Err("unexpected non-digit after '='".into())
                }
            }
            InstructionOp::Remove => Err("unexpected character after '-'".into()),
        }
    }
}

struct Day15 {
    buckets: Vec<Vec<(u64, usize)>>,
    current_instruction: Instruction,
    total_instruction_hash: u32,
}

impl Day15 {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 256],
            total_instruction_hash: 0,
            current_instruction: Default::default(),
        }
    }
    fn end_instruction(&mut self) -> Result<(), Box<dyn Error>> {
        let instruction = take(&mut self.current_instruction);
        self.total_instruction_hash += instruction.hash as u32;
        let bucket = &mut self.buckets[instruction.bucket as usize];
        let entry_pos = bucket.iter().position(|entry| entry.0 == instruction.key);
        match instruction.op {
            InstructionOp::Incomplete => Err("incomplete instruction".into()),
            InstructionOp::Set(value) => {
                if let Some(pos) = entry_pos {
                    bucket[pos].1 = value;
                } else {
                    bucket.push((instruction.key, value));
                }
                Ok(())
            }
            InstructionOp::Remove => {
                if let Some(pos) = entry_pos {
                    bucket.remove(pos);
                }
                Ok(())
            }
        }
    }
    fn total_power(&self) -> usize {
        let mut total = 0;
        for (bucket, bucket_power) in self.buckets.iter().zip(1usize..) {
            let mut bucket_total = 0;
            for ((_, value), slot_power) in bucket.iter().zip(1usize..) {
                bucket_total += value * slot_power;
            }
            total += bucket_total * bucket_power;
        }
        total
    }
}

impl LineStreamHandler for Day15 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        for ch in line.chars() {
            if ch == ',' {
                self.end_instruction()?;
            } else {
                self.current_instruction.update(ch)?;
            }
        }
        self.end_instruction()?;
        println!(
            "[{}] Sum of instruction HASHes: {}",
            SILVER_ANSI, self.total_instruction_hash
        );
        println!(
            "[{}] Sum of lens powers:        {}",
            GOLD_ANSI,
            self.total_power()
        );
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(15, "Lens Library", Day15::new()))
}
