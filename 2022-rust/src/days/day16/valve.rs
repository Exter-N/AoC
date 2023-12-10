use std::fmt::Debug;

use aoc_common_rs::cc::TwoCC;
use aoc_common_rs::ord::ProximityMap;

#[derive(Clone, Debug, Default)]
pub struct Valve {
    pub flow_rate: u32,
    pub neighbors: ProximityMap<TwoCC, u16>,
}

impl Valve {
    pub fn new(flow_rate: u32, neighbors: ProximityMap<TwoCC, u16>) -> Self {
        Self {
            flow_rate,
            neighbors,
            ..Default::default()
        }
    }
    pub fn is_jammed(&self) -> bool {
        0 == self.flow_rate
    }
}

pub const STARTING_VALVE_ID: TwoCC = TwoCC::new('A', 'A');
pub const VIRTUAL_STARTER_ID: TwoCC = TwoCC(0);
