use nom::{
    character::complete::{anychar, char},
    combinator::map,
    error::Error as NomError,
    sequence::{pair, separated_pair},
};

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_common_rs::{
    cc::TwoCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

struct Day23 {
    connections: HashMap<TwoCC, HashSet<TwoCC>>,
}

impl Day23 {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn is_connected(&self, id1: TwoCC, id2: TwoCC) -> bool {
        if let Some(peers) = self.connections.get(&id1.min(id2)) {
            peers.contains(&id1.max(id2))
        } else {
            false
        }
    }

    fn find_triples(&self) -> HashSet<Vec<TwoCC>> {
        let mut triples = HashSet::new();
        for (id1, peers) in self.connections.iter() {
            for id2 in peers {
                if let Some(peers2) = self.connections.get(id2) {
                    for id3 in peers {
                        if peers2.contains(id3) {
                            triples.insert(vec![*id1, *id2, *id3]);
                        }
                    }
                }
            }
        }
        triples
    }

    fn find_larger_meshes(&self, meshes: &HashSet<Vec<TwoCC>>) -> HashSet<Vec<TwoCC>> {
        let mut larger_meshes = HashSet::new();
        for id in self.connections.keys() {
            for mesh in meshes {
                if mesh.contains(id) {
                    continue;
                }

                if mesh.iter().all(|id2| self.is_connected(*id, *id2)) {
                    let mut larger_mesh = mesh.clone();
                    if let Err(pos) = larger_mesh.binary_search(id) {
                        larger_mesh.insert(pos, *id);
                    }
                    larger_meshes.insert(larger_mesh);
                }
            }
        }
        larger_meshes
    }
}

impl LineStreamHandler for Day23 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let two_cc = move || map(pair(anychar::<&str, NomError<&str>>, anychar), TwoCC::from);
        let (id1, id2) = parse_full_string(line, separated_pair(two_cc(), char('-'), two_cc()))?;
        self.connections
            .entry(id1.min(id2))
            .or_insert_with(|| HashSet::new())
            .insert(id1.max(id2));
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let triples = self.find_triples();
        let triples_with_t = triples
            .iter()
            .filter(|triple| triple.iter().any(|id| id.first() == 't'))
            .count();
        println!("[{}] Triples with a t: {}", SILVER_ANSI, triples_with_t);
        println!("[-] Meshes of  3:     {}", triples.len());
        let mut meshes = triples;
        let mut mesh_size = 3usize;
        loop {
            let larger_meshes = self.find_larger_meshes(&meshes);
            if larger_meshes.is_empty() {
                break;
            }
            meshes = larger_meshes;
            mesh_size += 1;
            println!("[-] Meshes of {:2}:     {}", mesh_size, meshes.len());
        }
        for mesh in meshes {
            print!("[{}] Largest mesh:     ", GOLD_ANSI);
            let mut first = true;
            for id in mesh {
                if first {
                    first = false;
                    print!("{}", id);
                } else {
                    print!(",{}", id);
                }
            }
            println!();
        }
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(23, "LAN Party", Day23::new()))
}
