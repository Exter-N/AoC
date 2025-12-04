use std::{
    error::Error,
    fmt::{Display, Write},
    u8,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    terrain::Terrain,
};

#[derive(Clone, Copy, Debug)]
enum Cell {
    Free,
    Roll { neighbors: u8 },
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Free => '.',
            Cell::Roll { neighbors } => {
                if *neighbors < 4 {
                    'x'
                } else {
                    '@'
                }
            }
        })
    }
}

struct Day4 {
    wall: Terrain<Cell>,
    verbose: bool,
}

impl Day4 {
    fn new(verbose: bool) -> Self {
        Self {
            wall: Terrain::new(),
            verbose,
        }
    }

    fn calculate_neighbors(&mut self) {
        for pt in self.wall.points() {
            if !matches!(self.wall[pt], Cell::Roll { neighbors: _ }) {
                continue;
            }

            let mut neighbors = 0u8;
            for (dir, neighbor) in self.wall.neighbors(pt) {
                if matches!(self.wall[neighbor], Cell::Roll { neighbors: _ }) {
                    neighbors += 1;
                }
                if let Some(diagonal) = self.wall.neighbor(neighbor, dir.clockwise()) {
                    if matches!(self.wall[diagonal], Cell::Roll { neighbors: _ }) {
                        neighbors += 1;
                    }
                }
            }

            self.wall[pt] = Cell::Roll { neighbors };
        }
    }

    fn remove_rolls(&mut self) -> usize {
        let mut rolls = 0;
        for pt in self.wall.points() {
            if let Cell::Roll { neighbors } = self.wall[pt] {
                if neighbors < 4 {
                    self.wall[pt] = Cell::Free;
                    rolls += 1;
                }
            }
        }

        rolls
    }
}

impl LineStreamHandler for Day4 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.wall.new_row();
        for ch in line.chars() {
            row.push(match ch {
                '.' => Cell::Free,
                '@' => Cell::Roll { neighbors: u8::MAX },
                _ => panic!("Unrecognized symbol {}", ch),
            });
        }
        self.wall.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.calculate_neighbors();
        let rolls = self.remove_rolls();
        println!("[{}] Accessible rolls:      {}", SILVER_ANSI, rolls);
        let mut rolls_this_round = rolls;
        let mut total_rolls = rolls;
        while rolls_this_round > 0 {
            self.calculate_neighbors();
            rolls_this_round = self.remove_rolls();
            total_rolls += rolls_this_round;
        }
        println!("[{}] Total removable rolls: {}", GOLD_ANSI, total_rolls);
        if self.verbose {
            println!("{}", self.wall);
        }
        Ok(())
    }
}

pub fn new(verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(4, "Printing Department", Day4::new(verbose)))
}
