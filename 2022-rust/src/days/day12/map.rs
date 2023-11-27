use std::error::Error;

use crate::point::{Direction2, Point2};
use crate::terrain::Terrain;

#[derive(Debug, Default)]
#[repr(u8)]
pub enum PathEnd {
    #[default]
    None = 0,
    Start = 1,
    End = 2,
}

#[derive(Debug, Default)]
pub struct TerrainCell {
    height: u8,
    path_end: PathEnd,
    distance: Option<usize>,
    towards_start: Option<Direction2>,
    towards_end: Option<Direction2>,
}

impl TerrainCell {
    fn new(height: u8, path_end: PathEnd) -> Self {
        Self {
            height,
            path_end,
            ..Default::default()
        }
    }
}

impl TryFrom<char> for TerrainCell {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'S' {
            Ok(Self::new(1, PathEnd::Start))
        } else if value == 'E' {
            Ok(Self::new(26, PathEnd::End))
        } else if value >= 'a' && value <= 'z' {
            Ok(Self::new(value as u8 - 'a' as u8 + 1, PathEnd::None))
        } else {
            Err(Box::from("invalid terrain character"))
        }
    }
}

impl From<&TerrainCell> for char {
    fn from(value: &TerrainCell) -> Self {
        if let Some(direction) = value.towards_end {
            match direction {
                Direction2::Right => '→',
                Direction2::Down => '↓',
                Direction2::Left => '←',
                Direction2::Up => '↑',
            }
        } else {
            match value.path_end {
                PathEnd::Start => 'S',
                PathEnd::End => 'E',
                PathEnd::None => (value.height - 1 + 'a' as u8) as char,
            }
        }
    }
}

#[derive(Default)]
pub struct Map {
    terrain: Terrain<TerrainCell>,
    start: Point2<usize>,
    end: Point2<usize>,
    path: Vec<Point2<usize>>,
}

impl Map {
    pub fn new_row(&self) -> Vec<TerrainCell> {
        self.terrain.new_row()
    }
    pub fn push_row(&mut self, row: Vec<TerrainCell>) {
        self.terrain.push_row(row);
    }
    pub fn update(&mut self, cell: &TerrainCell, j: usize) {
        match cell.path_end {
            PathEnd::None => {}
            PathEnd::Start => {
                self.start = Point2(j, self.terrain.height());
            }
            PathEnd::End => {
                self.end = Point2(j, self.terrain.height());
            }
        }
    }
    pub fn climbing_distance(&self) -> Option<usize> {
        self.terrain[self.end].distance
    }
    pub fn calculate_distances(&mut self, from_any: bool) {
        let mut queue: Vec<Point2<usize>> = Vec::new();
        if from_any {
            for pt in self.terrain.points() {
                if self.terrain[pt].height == 1 {
                    queue.push(pt);
                }
            }
        } else {
            queue.push(self.start);
        }
        for pt in queue.iter() {
            self.terrain[*pt].distance = Some(0);
        }
        for distance in 1usize.. {
            let pts = queue.split_off(0);
            if pts.is_empty() {
                break;
            }
            for pt in pts {
                let height = self.terrain[pt].height;
                for (direction, neighbor) in self.terrain.neighbors(pt).collect::<Vec<_>>() {
                    if self.terrain[neighbor].height <= height + 1
                        && self.terrain[neighbor].distance.is_none()
                    {
                        self.terrain[neighbor].distance = Some(distance);
                        self.terrain[neighbor].towards_start = Some(-direction);
                        queue.push(neighbor);
                    }
                }
            }
        }
    }
    pub fn calculate_path(&mut self) {
        let mut rev_path: Vec<Point2<usize>> = Vec::new();
        let mut pt = self.end;
        loop {
            rev_path.push(pt);
            if let Some(direction) = self.terrain[pt].towards_start {
                pt = pt.next_towards(direction);
                self.terrain[pt].towards_end = Some(-direction);
            } else {
                break;
            }
        }
        self.path.reserve(rev_path.len());
        for pt in rev_path.iter().rev() {
            self.path.push(*pt);
        }
    }
    pub fn dump(&self) {
        for row in self.terrain.iter() {
            for cell in row {
                print!("{}", char::from(cell));
            }
            println!();
        }
    }
}
