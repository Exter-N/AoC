use std::{collections::HashSet, error::Error};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pipe {
    None,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl Pipe {
    fn links_to(self, direction: Direction2) -> bool {
        match direction {
            Direction2::Up => matches!(self, Self::Vertical | Self::NorthEast | Self::NorthWest),
            Direction2::Down => matches!(self, Self::Vertical | Self::SouthWest | Self::SouthEast),
            Direction2::Left => {
                matches!(self, Self::Horizontal | Self::NorthWest | Self::SouthWest)
            }
            Direction2::Right => {
                matches!(self, Self::Horizontal | Self::NorthEast | Self::SouthEast)
            }
        }
    }
    fn from_links(to_north: bool, to_east: bool, to_west: bool) -> Result<Self, Box<dyn Error>> {
        match (to_north, to_east, to_west) {
            (true, false, false) => Ok(Self::Vertical),
            (false, true, true) => Ok(Self::Horizontal),
            (true, true, false) => Ok(Self::NorthEast),
            (true, false, true) => Ok(Self::NorthWest),
            (false, false, true) => Ok(Self::SouthWest),
            (false, true, false) => Ok(Self::SouthEast),
            _ => Err("cannot determine pipe from links".into()),
        }
    }
    fn any_direction(self) -> Option<Direction2> {
        match self {
            Self::None | Self::Start => None,
            Self::Vertical | Self::NorthEast | Self::NorthWest => Some(Direction2::Up),
            Self::Horizontal | Self::SouthWest => Some(Direction2::Left),
            Self::SouthEast => Some(Direction2::Right),
        }
    }
    fn direction_through(self, incoming_direction: Direction2) -> Option<Direction2> {
        match (self, incoming_direction) {
            (Self::Vertical, Direction2::Up) => Some(Direction2::Up),
            (Self::Vertical, Direction2::Down) => Some(Direction2::Down),
            (Self::Horizontal, Direction2::Left) => Some(Direction2::Left),
            (Self::Horizontal, Direction2::Right) => Some(Direction2::Right),
            (Self::NorthEast, Direction2::Down) => Some(Direction2::Right),
            (Self::NorthEast, Direction2::Left) => Some(Direction2::Up),
            (Self::NorthWest, Direction2::Down) => Some(Direction2::Left),
            (Self::NorthWest, Direction2::Right) => Some(Direction2::Up),
            (Self::SouthWest, Direction2::Up) => Some(Direction2::Left),
            (Self::SouthWest, Direction2::Right) => Some(Direction2::Down),
            (Self::SouthEast, Direction2::Up) => Some(Direction2::Right),
            (Self::SouthEast, Direction2::Left) => Some(Direction2::Down),
            _ => None,
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::None),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            'S' => Ok(Self::Start),
            _ => Err("unrecognized character".into()),
        }
    }
}

struct Day10 {
    terrain: Terrain<Pipe>,
    start: Option<Point2<usize>>,
}

impl Day10 {
    fn new() -> Self {
        Self {
            terrain: Terrain::new(),
            start: None,
        }
    }
    fn main_loop(&self) -> Result<Vec<Point2<usize>>, Box<dyn Error>> {
        if let Some(start) = self.start {
            let mut direction = self.terrain[start]
                .any_direction()
                .ok_or_else(|| Box::<dyn Error>::from("no start direction"))?;
            let mut main_loop = vec![start];
            let mut current = start.next_towards(direction);
            while current != start {
                direction = self.terrain[current]
                    .direction_through(direction)
                    .ok_or_else(|| Box::<dyn Error>::from("dead end"))?;
                main_loop.push(current);
                current = current.next_towards(direction);
            }
            Ok(main_loop)
        } else {
            Err("no start position".into())
        }
    }
    fn enclosed_by(&self, perimeter: HashSet<Point2<usize>>) -> HashSet<Point2<usize>> {
        let width = self.terrain.width();
        let height = self.terrain.height();
        let mut inbetweens = Terrain::new_with(width + 1, height + 1, |_| true);
        inbetweens[Point2(0, 0)] = false;
        let mut next: Vec<Point2<usize>> = vec![Point2(0, 0)];
        while next.len() > 0 {
            let current = next;
            next = Vec::new();
            for point in current.into_iter() {
                for (direction, neighbor) in inbetweens.neighbors(point) {
                    if !inbetweens[neighbor] {
                        continue;
                    }
                    let ref_point = match direction {
                        Direction2::Right | Direction2::Down => point,
                        Direction2::Left | Direction2::Up => neighbor,
                    };
                    if match direction {
                        Direction2::Right | Direction2::Left => {
                            ref_point.1 > 0
                                && perimeter.contains(&ref_point.next_up())
                                && perimeter.contains(&ref_point)
                                && self.terrain[ref_point.next_up()].links_to(Direction2::Down)
                                && self.terrain[ref_point].links_to(Direction2::Up)
                        }
                        Direction2::Down | Direction2::Up => {
                            ref_point.0 > 0
                                && perimeter.contains(&ref_point.next_left())
                                && perimeter.contains(&ref_point)
                                && self.terrain[ref_point.next_left()].links_to(Direction2::Right)
                                && self.terrain[ref_point].links_to(Direction2::Left)
                        }
                    } {
                        continue;
                    }
                    inbetweens[neighbor] = false;
                    next.push(neighbor);
                }
            }
        }
        let mut interior = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                if !perimeter.contains(&Point2(x, y))
                    && inbetweens[Point2(x, y)]
                    && inbetweens[Point2(x + 1, y)]
                    && inbetweens[Point2(x, y + 1)]
                    && inbetweens[Point2(x + 1, y + 1)]
                {
                    interior.insert(Point2(x, y));
                }
            }
        }
        interior
    }
}

impl LineStreamHandler for Day10 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = line
            .chars()
            .map(Pipe::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(start) = row.iter().position(|dir| matches!(*dir, Pipe::Start)) {
            if self.start.is_some() {
                return Err("multiple start positions".into());
            }
            let start_row = self.terrain.len();
            self.start = Some(Point2(start, start_row));
            let links_to_north = start_row > 0
                && self.terrain[Point2(start, start_row - 1)].links_to(Direction2::Down);
            let links_to_west = start > 0 && row[start - 1].links_to(Direction2::Right);
            let links_to_east = start + 1 < row.len() && row[start + 1].links_to(Direction2::Left);
            row[start] = Pipe::from_links(links_to_north, links_to_east, links_to_west)?;
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let main_loop = self.main_loop()?;
        println!(
            "[{}] Farthest distance from start: {}",
            SILVER_ANSI,
            main_loop.len() / 2
        );
        let inside = self.enclosed_by(main_loop.into_iter().collect());
        println!(
            "[{}] Tiles enclosed by main loop:  {}",
            GOLD_ANSI,
            inside.len()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(10, "Pipe Maze", Day10::new()))
}
