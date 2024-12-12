use std::{cell::RefCell, error::Error, usize};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::Direction2,
    terrain::Terrain,
};

#[derive(Debug)]
struct Plot {
    plant: char,
    region_id: usize,
}

impl Plot {
    fn new(plant: char) -> Self {
        Self {
            plant,
            region_id: usize::MAX,
        }
    }

    fn has_region(&self) -> bool {
        self.region_id != usize::MAX
    }
}

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn new() -> Self {
        Self {
            area: 0,
            perimeter: 0,
            sides: 0,
        }
    }

    fn fence_cost(&self) -> usize {
        self.area * self.perimeter
    }

    fn discounted_fence_cost(&self) -> usize {
        self.area * self.sides
    }
}

#[derive(Debug)]
struct Day12 {
    garden: Terrain<Plot>,
    regions: Vec<Region>,
}

impl Day12 {
    fn new() -> Self {
        Self {
            garden: Terrain::new(),
            regions: Vec::new(),
        }
    }

    fn calculate_regions(&mut self) {
        for pt in self.garden.points() {
            if self.garden[pt].has_region() {
                continue;
            }

            let plant = self.garden[pt].plant;
            let region = RefCell::new(Region::new());
            let region_id = self.regions.len();
            self.garden.flood_fill_mut(
                pt,
                |garden, _, pt2, dir, neighbor| {
                    let expand = garden[neighbor].plant == plant;
                    if !expand {
                        let mut rgn = region.borrow_mut();
                        rgn.perimeter += 1;
                        match garden.neighbor(pt2, dir.counterclockwise()) {
                            Some(side_neighbor) => {
                                if garden[side_neighbor].plant != plant
                                    || garden[garden.neighbor(side_neighbor, dir).unwrap()].plant
                                        == plant
                                {
                                    rgn.sides += 1;
                                }
                            }
                            None => {
                                rgn.sides += 1;
                            }
                        }
                    }
                    expand
                },
                |garden, _, pt2| {
                    garden[pt2].region_id = region_id;
                    let mut rgn = region.borrow_mut();
                    rgn.area += 1;
                    for dir in Direction2::all() {
                        if garden.neighbor(pt2, dir).is_some() {
                            continue;
                        }
                        rgn.perimeter += 1;
                        match garden.neighbor(pt2, dir.counterclockwise()) {
                            Some(side_neighbor) => {
                                if garden[side_neighbor].plant != plant {
                                    rgn.sides += 1;
                                }
                            }
                            None => {
                                rgn.sides += 1;
                            }
                        }
                    }
                },
            );
            self.regions.push(region.into_inner());
        }
    }

    fn fence_cost(&self) -> usize {
        let mut cost = 0;
        for region in &self.regions {
            cost += region.fence_cost();
        }
        cost
    }

    fn discounted_fence_cost(&self) -> usize {
        let mut cost = 0;
        for region in &self.regions {
            cost += region.discounted_fence_cost();
        }
        cost
    }
}

impl LineStreamHandler for Day12 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.garden.new_row();
        for ch in line.chars() {
            row.push(Plot::new(ch));
        }
        self.garden.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.calculate_regions();
        println!(
            "[{}] Total fence cost:      {}",
            SILVER_ANSI,
            self.fence_cost()
        );
        println!(
            "[{}] Discounted fence cost: {}",
            GOLD_ANSI,
            self.discounted_fence_cost()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(12, "Garden Groups", Day12::new()))
}
