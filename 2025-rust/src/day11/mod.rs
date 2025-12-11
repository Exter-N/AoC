use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_common_rs::{
    cc::ThreeCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string, take_fixed},
    some_or_break,
};
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::char, combinator::map, multi::separated_list1,
    sequence::separated_pair,
};

const YOU_3CC: ThreeCC = ThreeCC::new('y', 'o', 'u');
const SVR_3CC: ThreeCC = ThreeCC::new('s', 'v', 'r');
const DAC_3CC: ThreeCC = ThreeCC::new('d', 'a', 'c');
const FFT_3CC: ThreeCC = ThreeCC::new('f', 'f', 't');
const OUT_3CC: ThreeCC = ThreeCC::new('o', 'u', 't');

struct Day11 {
    devices: HashMap<ThreeCC, usize>,
    outputs: Vec<Vec<usize>>,
}

impl Day11 {
    fn new() -> Self {
        Self {
            devices: HashMap::new(),
            outputs: Vec::new(),
        }
    }

    fn get_device_index(&self, device: ThreeCC) -> Option<usize> {
        self.devices.get(&device).map(|index| *index)
    }

    fn get_or_create_device_index(&mut self, device: ThreeCC) -> usize {
        let len = self.devices.len();
        let index = *self.devices.entry(device).or_insert(len);
        if index == len {
            self.outputs.push(Vec::new());
        }
        index
    }

    fn next_state(&self, state: Vec<usize>) -> Option<Vec<usize>> {
        let mut any = false;
        let mut next_state = vec![0usize; self.outputs.len()];
        for (paths, device) in state.into_iter().zip(0usize..) {
            if paths > 0 {
                for output in self.outputs[device].iter() {
                    next_state[*output] += paths;
                    any = true;
                }
            }
        }
        if any { Some(next_state) } else { None }
    }

    fn paths_from_to(&self, from: ThreeCC, to: ThreeCC) -> usize {
        let Some(from_index) = self.get_device_index(from) else {
            return 0;
        };
        let Some(to_index) = self.get_device_index(to) else {
            return 0;
        };
        let mut total_paths = 0usize;
        let mut state = vec![0usize; self.outputs.len()];
        state[from_index] = 1;
        loop {
            state = some_or_break!(self.next_state(state));
            total_paths += state[to_index];
            state[to_index] = 0;
        }
        total_paths
    }
}

impl LineStreamHandler for Day11 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (device, outputs) = parse_full_string(
            line,
            separated_pair(
                map(take_fixed::<3, _>(), ThreeCC::from),
                tag(": "),
                map(
                    separated_list1(char(' '), map(take_fixed::<3, _>(), ThreeCC::from)),
                    |vec| vec.into_iter().collect::<HashSet<_>>(),
                ),
            ),
        )?;
        let device_index = self.get_or_create_device_index(device);
        let mut output_indices = outputs
            .into_iter()
            .map(|output| self.get_or_create_device_index(output))
            .collect_vec();
        output_indices.sort();
        self.outputs[device_index] = output_indices;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Paths from you to out: {}",
            SILVER_ANSI,
            self.paths_from_to(YOU_3CC, OUT_3CC)
        );
        let fft_to_dac = self.paths_from_to(FFT_3CC, DAC_3CC);
        let dac_to_fft = self.paths_from_to(DAC_3CC, FFT_3CC);
        if fft_to_dac > 0 {
            if dac_to_fft > 0 {
                unreachable!();
            }
            let srv_to_fft = self.paths_from_to(SVR_3CC, FFT_3CC);
            let dac_to_out = self.paths_from_to(DAC_3CC, OUT_3CC);
            println!("[-] Paths from srv to fft: {}", srv_to_fft);
            println!("[-] Paths from fft to dac: {}", fft_to_dac);
            println!("[-] Paths from dac to out: {}", dac_to_out);
            println!(
                "[{}] Full paths:            {}",
                GOLD_ANSI,
                srv_to_fft * fft_to_dac * dac_to_out
            );
        } else if dac_to_fft > 0 {
            let srv_to_dac = self.paths_from_to(SVR_3CC, DAC_3CC);
            let fft_to_out = self.paths_from_to(FFT_3CC, OUT_3CC);
            println!("[-] Paths from srv to dac: {}", srv_to_dac);
            println!("[-] Paths from dac to fft: {}", dac_to_fft);
            println!("[-] Paths from fft to out: {}", fft_to_out);
            println!(
                "[{}] Full paths:            {}",
                GOLD_ANSI,
                srv_to_dac * dac_to_fft * fft_to_out
            );
        }
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(11, "Reactor", Day11::new()))
}
