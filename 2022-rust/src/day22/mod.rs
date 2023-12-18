use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

use nom::branch::alt;
use nom::character::complete::{char, u32};
use nom::combinator::map;
use nom::multi::many0;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandlerOnce},
    math::gcd,
    point::{Direction2, Direction3, Point2},
};

#[derive(Debug, Default)]
struct Chunk {
    data: Vec<Vec<bool>>,
    neighbors: HashMap<Direction2, ChunkNeighbor>,
}

impl Chunk {
    fn new() -> Self {
        Default::default()
    }
    fn is_wall(&self, pos: Point2<usize>) -> bool {
        self.data[pos.1 - 1][pos.0 - 1]
    }
}

#[derive(Debug)]
struct ChunkNeighbor {
    chunk: Point2<usize>,
    edge: Direction2,
    reverse: bool,
}

impl ChunkNeighbor {
    fn new(chunk: Point2<usize>, edge: Direction2, reverse: bool) -> Self {
        Self {
            chunk,
            edge,
            reverse,
        }
    }
}

#[derive(Debug)]
struct Day22 {
    fold_as_cube: bool,
    chunk_size: Option<usize>,
    height: usize,
    chunks: HashMap<Point2<usize>, Chunk>,
    position: (Point2<usize>, Point2<usize>, Direction2),
}

impl Day22 {
    fn new(fold_as_cube: bool) -> Self {
        Self {
            fold_as_cube,
            chunk_size: None,
            height: 0,
            chunks: HashMap::new(),
            position: (Point2(0, 0), Point2(0, 0), Direction2::Right),
        }
    }
    fn bind_chunks(
        &mut self,
        pos: Point2<usize>,
        dir: Direction2,
        rev_pos: Point2<usize>,
        rev_dir: Direction2,
        reverse: bool,
    ) {
        self.chunks
            .get_mut(&pos)
            .unwrap()
            .neighbors
            .insert(dir, ChunkNeighbor::new(rev_pos, rev_dir, reverse));
        self.chunks
            .get_mut(&rev_pos)
            .unwrap()
            .neighbors
            .insert(rev_dir, ChunkNeighbor::new(pos, dir, reverse));
    }
    fn fold(&mut self) {
        let chunk_keys = self.chunks.keys().map(|pt| *pt).collect::<HashSet<_>>();
        for pos in chunk_keys.iter() {
            for dir in Direction2::all() {
                let pos_n = pos.next_towards(dir);
                if chunk_keys.contains(&pos_n) {
                    self.chunks
                        .get_mut(&pos)
                        .unwrap()
                        .neighbors
                        .insert(dir, ChunkNeighbor::new(pos_n, -dir, false));
                }
            }
        }
        if self.fold_as_cube {
            let mut bases: HashMap<Point2<usize>, (Direction3, Direction3)> = HashMap::new();
            let mut faces: HashMap<Direction3, Point2<usize>> = HashMap::new();
            let mut queue: VecDeque<Point2<usize>> = VecDeque::new();
            bases.insert(self.position.0, (Direction3::Right, Direction3::Down));
            faces.insert(Direction3::Back, self.position.0);
            queue.push_back(self.position.0);
            while let Some(pos) = queue.pop_front() {
                let basis = bases.get(&pos).unwrap().to_owned();
                for dir in Direction2::all() {
                    if let Some(neighbor) = self.chunks.get(&pos).unwrap().neighbors.get(&dir) {
                        if !bases.contains_key(&neighbor.chunk) {
                            let basis_n = neighbor_basis(basis, dir).unwrap();
                            bases.insert(neighbor.chunk, basis_n);
                            faces.insert(basis_n.0.cross(basis_n.1).unwrap(), neighbor.chunk);
                            queue.push_back(neighbor.chunk);
                        }
                    }
                }
            }
            for pos in chunk_keys.iter() {
                let basis = bases.get(&pos).unwrap().to_owned();
                for dir in Direction2::all() {
                    if !self.chunks.get(pos).unwrap().neighbors.contains_key(&dir) {
                        let basis_n = neighbor_basis(basis, dir).unwrap();
                        if let Some(pos_n) = faces.get(&basis_n.0.cross(basis_n.1).unwrap()) {
                            if let Some(basis_ch) = bases.get(pos_n) {
                                let rev_dir = neighbor_edge(basis, *basis_ch).unwrap();
                                let along = direction_along_edge(basis, dir);
                                let rev_along = direction_along_edge(*basis_ch, rev_dir);
                                let reverse = if along == rev_along {
                                    false
                                } else if along == -rev_along {
                                    true
                                } else {
                                    panic!(
                                        "directions along edge don't match: {:?} and {:?}",
                                        along, rev_along
                                    );
                                };
                                self.bind_chunks(*pos, dir, *pos_n, rev_dir, reverse);
                            }
                        }
                    }
                }
            }
        } else {
            for pos in chunk_keys.iter() {
                for dir in Direction2::all() {
                    if !self.chunks.get(pos).unwrap().neighbors.contains_key(&dir) {
                        let mut pt_wrap = *pos;
                        while let Some(neighbor) =
                            self.chunks.get(&pt_wrap).unwrap().neighbors.get(&-dir)
                        {
                            pt_wrap = neighbor.chunk;
                        }
                        self.bind_chunks(*pos, dir, pt_wrap, -dir, false);
                    }
                }
            }
        }
        for pos in chunk_keys {
            if self.chunks.get(&pos).unwrap().neighbors.len() != 4 {
                panic!("failed to fold, {:?} doesn't have 4 neighbors", pos);
            }
        }
    }
    fn do_move(&mut self, m: Move) {
        match m {
            Move::Walk(steps) => {
                let chunk_size = self.chunk_size.unwrap();
                for _ in 0..steps {
                    let next = self.position.1.next_towards(self.position.2);
                    let overflow = if next.0 == 0 {
                        Some((Direction2::Left, next.1))
                    } else if next.1 == 0 {
                        Some((Direction2::Up, next.0))
                    } else if next.0 == chunk_size + 1 {
                        Some((Direction2::Right, next.1))
                    } else if next.1 == chunk_size + 1 {
                        Some((Direction2::Down, next.0))
                    } else {
                        None
                    };
                    let next_pos = if let Some(ovf) = overflow {
                        let chunk = self.chunks.get(&self.position.0).unwrap();
                        let neighbor = chunk.neighbors.get(&ovf.0).unwrap();
                        let along = if neighbor.reverse {
                            chunk_size + 1 - ovf.1
                        } else {
                            ovf.1
                        };
                        (
                            neighbor.chunk,
                            match neighbor.edge {
                                Direction2::Right => Point2(chunk_size, along),
                                Direction2::Down => Point2(along, chunk_size),
                                Direction2::Left => Point2(1, along),
                                Direction2::Up => Point2(along, 1),
                            },
                            -neighbor.edge,
                        )
                    } else {
                        (self.position.0, next, self.position.2)
                    };
                    if !self.chunks.get(&next_pos.0).unwrap().is_wall(next_pos.1) {
                        self.position = next_pos;
                    }
                }
            }
            Move::TurnCounterclockwise => {
                self.position.2 = self.position.2.counterclockwise();
            }
            Move::TurnClockwise => {
                self.position.2 = self.position.2.clockwise();
            }
        }
    }
    fn password(&self) -> u32 {
        let chunk_size = self.chunk_size.unwrap();
        let (chunk, pos, direction) = self.position;
        let y = (chunk.1 - 1) * chunk_size + pos.1;
        let x = (chunk.0 - 1) * chunk_size + pos.0;

        (y as u32) * 1000 + (x as u32) * 4 + (direction as u8 as u32)
    }
}

fn neighbor_basis(
    basis: (Direction3, Direction3),
    towards: Direction2,
) -> Option<(Direction3, Direction3)> {
    basis.0.cross(basis.1).map(|cross| match towards {
        Direction2::Right => (cross, basis.1),
        Direction2::Down => (basis.0, cross),
        Direction2::Left => (-cross, basis.1),
        Direction2::Up => (basis.0, -cross),
    })
}

fn neighbor_edge(
    from_basis: (Direction3, Direction3),
    to_basis: (Direction3, Direction3),
) -> Option<Direction2> {
    from_basis.0.cross(from_basis.1).and_then(|from_in| {
        if from_in == to_basis.0 {
            Some(Direction2::Left)
        } else if from_in == to_basis.1 {
            Some(Direction2::Up)
        } else if from_in == -to_basis.0 {
            Some(Direction2::Right)
        } else if from_in == -to_basis.1 {
            Some(Direction2::Down)
        } else {
            None
        }
    })
}

fn direction_along_edge(basis: (Direction3, Direction3), edge: Direction2) -> Direction3 {
    match edge {
        Direction2::Right | Direction2::Left => basis.1,
        Direction2::Down | Direction2::Up => basis.0,
    }
}

enum Move {
    Walk(usize),
    TurnCounterclockwise,
    TurnClockwise,
}

struct Day22Map(Day22);

struct Day22Moves(Day22);

impl LineStreamHandlerOnce for Day22Map {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let state = &mut self.0;
        if line.is_empty() {
            state.fold();

            return Ok(Box::new(Day22Moves(self.0)));
        }
        if let Some(spaces) = line.chars().position(|ch| ch != ' ') {
            let chunk_size = state.chunk_size.unwrap_or_else(|| {
                let size = gcd(spaces, line.len());
                state.chunk_size = Some(size);
                state.position = (
                    Point2((spaces / size) + 1, 1),
                    Point2(1, 1),
                    Direction2::Right,
                );

                size
            });
            let y = state.height / chunk_size;
            let xs = (spaces / chunk_size)..(line.len() / chunk_size);
            for x in xs {
                if !state.chunks.contains_key(&Point2(x + 1, y + 1)) {
                    state.chunks.insert(Point2(x + 1, y + 1), Chunk::new());
                }
                let chunk = state.chunks.get_mut(&Point2(x + 1, y + 1)).unwrap();
                let mut row = Vec::new();
                for ch in line[(x * chunk_size)..((x + 1) * chunk_size)].chars() {
                    row.push(ch == '#');
                }
                chunk.data.push(row);
            }
        }
        state.height += 1;

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

impl LineStreamHandlerOnce for Day22Moves {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let moves = parse_full_string(
            line,
            many0(alt((
                map(u32, |num| Move::Walk(num as usize)),
                map(char('L'), |_| Move::TurnCounterclockwise),
                map(char('R'), |_| Move::TurnClockwise),
            ))),
        )?;
        let state = &mut self.0;
        for m in moves {
            state.do_move(m);
        }

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let state = self.0;
        println!(
            "[{}] Final password: {}",
            if state.fold_as_cube {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            state.password()
        );

        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(22, "Monkey Map", Day22Map(Day22::new(gold))))
}
