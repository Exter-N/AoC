use std::{collections::HashMap, error::Error, num::NonZeroUsize};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string},
    point::Point3,
};
use itertools::Itertools;
use nom::{
    character::complete::{char, u32},
    sequence::separated_pair,
};

fn euclidean_distance(pt1: &Point3<f32>, pt2: &Point3<f32>) -> f32 {
    let dx = pt1.0 - pt2.0;
    let dy = pt1.1 - pt2.1;
    let dz = pt1.2 - pt2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn replace_all<T: Copy + PartialEq>(vec: &mut Vec<T>, search: T, replacement: T) {
    for item in vec.iter_mut() {
        if *item == search {
            *item = replacement;
        }
    }
}

struct Day8 {
    junction_boxes: Vec<Point3<f32>>,
    circuit_ids: Vec<Option<NonZeroUsize>>,
    next_circuit_id: NonZeroUsize,
    circuits: usize,
    unconnected: usize,
    gold: bool,
}

impl Day8 {
    fn new(gold: bool) -> Self {
        Self {
            junction_boxes: Vec::new(),
            circuit_ids: Vec::new(),
            next_circuit_id: NonZeroUsize::new(1).unwrap(),
            circuits: 0,
            unconnected: 0,
            gold,
        }
    }

    fn ordered_pairs(&self) -> Vec<(usize, usize, f32)> {
        let mut pairs = self
            .junction_boxes
            .iter()
            .zip(0usize..)
            .cartesian_product(self.junction_boxes.iter().zip(0usize..))
            .filter_map(|((box1, i1), (box2, i2))| {
                if i1 >= i2 {
                    None
                } else {
                    Some((i1, i2, euclidean_distance(box1, box2)))
                }
            })
            .collect_vec();
        pairs.sort_by(|pair1, pair2| pair1.2.partial_cmp(&pair2.2).unwrap());
        pairs
    }

    fn connect(&mut self, box1: usize, box2: usize) {
        match (self.circuit_ids[box1], self.circuit_ids[box2]) {
            (None, None) => {
                self.circuit_ids[box1] = Some(self.next_circuit_id);
                self.circuit_ids[box2] = Some(self.next_circuit_id);
                self.next_circuit_id = self.next_circuit_id.checked_add(1).unwrap();
                self.unconnected -= 2;
                self.circuits += 1;
            }
            (None, Some(circuit)) => {
                self.circuit_ids[box1] = Some(circuit);
                self.unconnected -= 1;
            }
            (Some(circuit), None) => {
                self.circuit_ids[box2] = Some(circuit);
                self.unconnected -= 1;
            }
            (Some(circuit1), Some(circuit2)) => {
                if circuit1 < circuit2 {
                    replace_all(&mut self.circuit_ids, Some(circuit2), Some(circuit1));
                    self.circuits -= 1;
                } else if circuit2 < circuit1 {
                    replace_all(&mut self.circuit_ids, Some(circuit1), Some(circuit2));
                    self.circuits -= 1;
                }
            }
        }
    }

    fn get_circuit_sizes(&self) -> (Vec<(NonZeroUsize, usize)>, usize) {
        let mut circuits: HashMap<NonZeroUsize, usize> = HashMap::new();
        let mut circuitless = 0usize;
        for id in self.circuit_ids.iter() {
            if let Some(id) = id {
                circuits
                    .entry(*id)
                    .and_modify(|size| *size += 1)
                    .or_insert(1);
            } else {
                circuitless += 1;
            }
        }
        let mut circuits = circuits.into_iter().collect_vec();
        circuits.sort_by(|(id1, size1), (id2, size2)| size2.cmp(size1).then_with(|| id1.cmp(id2)));
        (circuits, circuitless)
    }
}

impl LineStreamHandler for Day8 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (x, (y, z)) = parse_full_string(
            line,
            separated_pair(u32, char(','), separated_pair(u32, char(','), u32)),
        )?;
        self.junction_boxes
            .push(Point3(x as f32, y as f32, z as f32));
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.circuit_ids.resize(self.junction_boxes.len(), None);
        self.unconnected = self.junction_boxes.len();
        let mut pairs = self.ordered_pairs();
        if self.gold {
            for pair in pairs {
                self.connect(pair.0, pair.1);
                if self.unconnected == 0 && self.circuits == 1 {
                    println!(
                        "[{}] Product of last Xs: {}",
                        GOLD_ANSI,
                        self.junction_boxes[pair.0].0 as u64 * self.junction_boxes[pair.1].0 as u64
                    );
                    break;
                }
            }
        } else {
            pairs.truncate(10usize.pow(self.junction_boxes.len().ilog10()));
            for pair in pairs {
                self.connect(pair.0, pair.1);
            }
            let (circuit_sizes, _) = self.get_circuit_sizes();
            println!(
                "[{}] Product of sizes of three largest circuits: {}",
                SILVER_ANSI,
                circuit_sizes
                    .into_iter()
                    .take(3)
                    .map(|(_, size)| size)
                    .product::<usize>()
            );
        }
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(8, "Playground", Day8::new(gold)))
}
