use std::{error::Error, mem::take};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Clone, Copy, Debug, Default)]
enum Device {
    #[default]
    None,
    HorizontalSplitter,
    VerticalSplitter,
    NeswMirror,
    NwseMirror,
}

impl Device {
    fn outbound_directions(
        self,
        inbound_direction: Direction2,
    ) -> (Direction2, Option<Direction2>) {
        match (self, inbound_direction) {
            (Self::None, _) => (inbound_direction, None),
            (Self::HorizontalSplitter, Direction2::Right | Direction2::Left) => {
                (inbound_direction, None)
            }
            (Self::HorizontalSplitter, Direction2::Down | Direction2::Up) => {
                (Direction2::Right, Some(Direction2::Left))
            }
            (Self::VerticalSplitter, Direction2::Right | Direction2::Left) => {
                (Direction2::Down, Some(Direction2::Up))
            }
            (Self::VerticalSplitter, Direction2::Down | Direction2::Up) => {
                (inbound_direction, None)
            }
            (Self::NeswMirror, Direction2::Right) => (Direction2::Up, None),
            (Self::NeswMirror, Direction2::Down) => (Direction2::Left, None),
            (Self::NeswMirror, Direction2::Left) => (Direction2::Down, None),
            (Self::NeswMirror, Direction2::Up) => (Direction2::Right, None),
            (Self::NwseMirror, Direction2::Right) => (Direction2::Down, None),
            (Self::NwseMirror, Direction2::Down) => (Direction2::Right, None),
            (Self::NwseMirror, Direction2::Left) => (Direction2::Up, None),
            (Self::NwseMirror, Direction2::Up) => (Direction2::Left, None),
        }
    }
}

impl TryFrom<char> for Device {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::None),
            '-' => Ok(Self::HorizontalSplitter),
            '|' => Ok(Self::VerticalSplitter),
            '/' => Ok(Self::NeswMirror),
            '\\' => Ok(Self::NwseMirror),
            _ => Err("unrecognized character".into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Tile {
    device: Device,
    energized: bool,
    emitted_right: bool,
    emitted_down: bool,
    emitted_left: bool,
    emitted_up: bool,
}

impl Tile {
    fn new(device: Device) -> Self {
        Self {
            device,
            ..Default::default()
        }
    }
    fn emitted(&self, direction: Direction2) -> bool {
        match direction {
            Direction2::Right => self.emitted_right,
            Direction2::Down => self.emitted_down,
            Direction2::Left => self.emitted_left,
            Direction2::Up => self.emitted_up,
        }
    }
    fn set_emitted(&mut self, direction: Direction2, emitted: bool) {
        match direction {
            Direction2::Right => {
                self.emitted_right = emitted;
            }
            Direction2::Down => {
                self.emitted_down = emitted;
            }
            Direction2::Left => {
                self.emitted_left = emitted;
            }
            Direction2::Up => {
                self.emitted_up = emitted;
            }
        }
    }
}

fn energize(terrain: &mut Terrain<Tile>, from: Point2<usize>, towards: Direction2) {
    let size = terrain.size();
    let mut next_beams: Vec<(Point2<usize>, Direction2)> = vec![(from, towards)];
    while next_beams.len() > 0 {
        for (pt, dir) in take(&mut next_beams) {
            let tile = &mut terrain[pt];
            tile.energized = true;
            let (out_dir1, opt_out_dir2) = tile.device.outbound_directions(dir);
            if !tile.emitted(out_dir1) {
                tile.set_emitted(out_dir1, true);
                if let Some(out_pt1) = pt.try_next_towards(out_dir1) {
                    if out_pt1 < size {
                        next_beams.push((out_pt1, out_dir1));
                    }
                }
            }
            if let Some(out_dir2) = opt_out_dir2 {
                if !tile.emitted(out_dir2) {
                    tile.set_emitted(out_dir2, true);
                    if let Some(out_pt2) = pt.try_next_towards(out_dir2) {
                        if out_pt2 < size {
                            next_beams.push((out_pt2, out_dir2));
                        }
                    }
                }
            }
        }
    }
}

fn count_energized(terrain: &Terrain<Tile>) -> usize {
    let mut num_energized = 0;
    for pt in terrain.points() {
        if terrain[pt].energized {
            num_energized += 1;
        }
    }
    num_energized
}

struct Day16 {
    gold: bool,
    terrain: Terrain<Tile>,
}

impl Day16 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            terrain: Terrain::new(),
        }
    }
}

impl LineStreamHandler for Day16 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(Tile::new(ch.try_into()?));
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            let mut max_energized = 0;
            for y in 0..self.terrain.height() {
                let mut r_terrain = self.terrain.clone();
                energize(&mut r_terrain, Point2(0, y), Direction2::Right);
                max_energized = max_energized.max(count_energized(&r_terrain));
                let mut l_terrain = self.terrain.clone();
                energize(
                    &mut l_terrain,
                    Point2(self.terrain.width() - 1, y),
                    Direction2::Left,
                );
                max_energized = max_energized.max(count_energized(&l_terrain));
            }
            for x in 0..self.terrain.width() {
                let mut d_terrain = self.terrain.clone();
                energize(&mut d_terrain, Point2(x, 0), Direction2::Down);
                max_energized = max_energized.max(count_energized(&d_terrain));
                let mut u_terrain = self.terrain.clone();
                energize(
                    &mut u_terrain,
                    Point2(x, self.terrain.height() - 1),
                    Direction2::Up,
                );
                max_energized = max_energized.max(count_energized(&u_terrain));
            }
            println!(
                "[{}] Optimal number of energized tiles: {}",
                GOLD_ANSI, max_energized
            );
        } else {
            energize(&mut self.terrain, Point2(0, 0), Direction2::Right);
            println!(
                "[{}] Number of energized tiles: {}",
                SILVER_ANSI,
                count_energized(&self.terrain)
            );
        }
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(16, "The Floor Will Be Lava", Day16::new(gold)))
}
