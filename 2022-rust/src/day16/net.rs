use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::{Deref, DerefMut};

use aoc_common_rs::cc::TwoCC;
use aoc_common_rs::ord::Top;

use super::valve::{Valve, STARTING_VALVE_ID, VIRTUAL_STARTER_ID};

#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct ValveNetwork(HashMap<TwoCC, Valve>);

impl ValveNetwork {
    pub fn add_virtual_starter(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(starting_valve) = self.0.get_mut(&STARTING_VALVE_ID) {
            starting_valve.neighbors.insert(VIRTUAL_STARTER_ID, 0);

            let mut virtual_starter: Valve = Default::default();
            virtual_starter.neighbors.insert(STARTING_VALVE_ID, 0);
            self.0.insert(VIRTUAL_STARTER_ID, virtual_starter);

            Ok(())
        } else {
            Err(Box::from("starting valve not found"))
        }
    }
    fn link_neighbors(&mut self, id: TwoCC, unlink_self: bool) -> Result<(), Box<dyn Error>> {
        let neighbors = self.0[&id].neighbors.clone();
        for (id1, t1) in neighbors.iter() {
            for (id2, t2) in neighbors.iter() {
                if id2 > id1 {
                    if let Some(v1) = self.0.get_mut(id1) {
                        v1.neighbors.insert(*id2, t1 + t2);
                    } else {
                        return Err(Box::from("lower neighbor valve not found"));
                    }
                    if let Some(v2) = self.0.get_mut(id2) {
                        v2.neighbors.insert(*id1, t1 + t2);
                    } else {
                        return Err(Box::from("upper neighbor valve not found"));
                    }
                }
            }
            if unlink_self {
                self.0.get_mut(id1).unwrap().neighbors.remove(&id);
            }
        }

        Ok(())
    }
    fn link_all_neighbors(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ids: HashSet<TwoCC> = HashSet::new();
        for id in self.0.keys() {
            ids.insert(*id);
        }
        for id in ids {
            self.link_neighbors(id, false)?;
        }

        Ok(())
    }
    fn remove_jammed(&mut self) -> Result<(), Box<dyn Error>> {
        let mut jammed: HashSet<TwoCC> = HashSet::new();
        for (id, valve) in self.0.iter() {
            if valve.is_jammed() {
                jammed.insert(*id);
            }
        }
        jammed.remove(&VIRTUAL_STARTER_ID);
        for id in jammed {
            self.link_neighbors(id, true)?;
            self.0.remove(&id);
        }

        Ok(())
    }
    pub fn prepare(&mut self) -> Result<(), Box<dyn Error>> {
        self.remove_jammed()?;
        for _ in 0..self.0.len() {
            self.link_all_neighbors()?;
        }

        Ok(())
    }
    pub fn max_pressure_release(
        &self,
        t: u16,
        num: usize,
    ) -> Result<(u32, Vec<(TwoCC, u16)>), Box<dyn Error>> {
        let mut valves: HashSet<TwoCC> = HashSet::new();
        for id in self.0.keys() {
            valves.insert(*id);
        }
        valves.remove(&VIRTUAL_STARTER_ID);

        self.max_pressure_release_from(vec![(VIRTUAL_STARTER_ID, t); num], valves, 0)
    }
    fn max_pressure_release_from(
        &self,
        actors: Vec<(TwoCC, u16)>,
        valves: HashSet<TwoCC>,
        depth: usize,
    ) -> Result<(u32, Vec<(TwoCC, u16)>), Box<dyn Error>> {
        // println!("{:indent$} {:?} {:?}", "", ids_and_ts, remaining_ids, indent = depth);
        let mut top_moves: Top<PressureReleaseMove, 12> = Default::default();
        for id in &valves {
            let valve = self.0.get(&id).unwrap();
            let mut mv: PressureReleaseMove = Default::default();
            for (actor, i) in actors.iter().zip(0usize..) {
                let distance = *valve.neighbors.get(&actor.0).unwrap();
                if distance >= actor.1 {
                    continue;
                }
                let remaining = actor.1 - distance - 1;
                let a_mv = PressureReleaseMove {
                    actor_index: i,
                    target_valve: *id,
                    time_cost: distance + 1,
                    remaining_time: remaining,
                    pressure_release: (remaining as u32) * valve.flow_rate,
                };
                if a_mv > mv {
                    mv = a_mv;
                }
            }
            top_moves.insert(mv);
        }

        let mut best_release: u32 = 0;
        let mut best_history: Vec<(TwoCC, u16)> = Vec::new();

        for (mv, rank) in top_moves.iter().rev().zip(0usize..) {
            if mv.is_default() {
                break;
            }
            if rank > 0 && rank + depth > 12 {
                break;
            }
            let mut subsequent_actors = actors.clone();
            subsequent_actors[mv.actor_index] = mv.actor();
            let mut subsequent_valves = valves.clone();
            subsequent_valves.remove(&mv.target_valve);
            let (subsequent_release, subsequent_history) =
                self.max_pressure_release_from(subsequent_actors, subsequent_valves, depth + 1)?;

            let total_release = mv.pressure_release + subsequent_release;
            if total_release > best_release {
                best_release = total_release;
                best_history = subsequent_history;
                best_history.insert(0, mv.actor());
            }
        }

        Ok((best_release, best_history))
    }
}

impl Deref for ValveNetwork {
    type Target = HashMap<TwoCC, Valve>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValveNetwork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PressureReleaseMove {
    actor_index: usize,
    target_valve: TwoCC,
    time_cost: u16,
    remaining_time: u16,
    pressure_release: u32,
}

impl PressureReleaseMove {
    fn actor(&self) -> (TwoCC, u16) {
        (self.target_valve, self.remaining_time)
    }
    fn is_default(&self) -> bool {
        0 == self.time_cost
    }
}

impl PartialOrd for PressureReleaseMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PressureReleaseMove {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_default() {
            Ordering::Less
        } else if other.is_default() {
            Ordering::Greater
        } else {
            (self.pressure_release / (self.time_cost as u32))
                .cmp(&(other.pressure_release / (other.time_cost as u32)))
        }
    }
}
