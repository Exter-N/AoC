use std::error::Error;

use crate::math::scale_u8;
use crate::point::Point2;
use crate::terrain::Terrain;

const MAX_HEIGHT: u8 = 10;

#[derive(Default)]
pub struct Tree {
    height: u8,
    visible: bool,
    scenic_score: usize,
}

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            ..Default::default()
        }
    }
    fn update_visibility(&mut self, max_height: &mut u8, visible_count: &mut usize) -> bool {
        if self.height > *max_height {
            *max_height = self.height;
            if !self.visible {
                self.visible = true;
                *visible_count += 1;
            }

            self.height < MAX_HEIGHT
        } else {
            true
        }
    }
}

impl TryFrom<char> for Tree {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value >= '0' && value <= '9' {
            Ok(Self::new(value as u8 - '0' as u8 + 1))
        } else {
            Err(Box::from("invalid tree character"))
        }
    }
}

impl From<&Tree> for char {
    fn from(value: &Tree) -> Self {
        (value.height - 1 + '0' as u8) as Self
    }
}

#[derive(Default)]
pub struct Map {
    terrain: Terrain<Tree>,
    pub visible_count: usize,
    pub max_visible_score: usize,
    pub max_hidden_score: usize,
}

impl Map {
    pub fn new_row(&self) -> Vec<Tree> {
        self.terrain.new_row()
    }
    pub fn push_row(&mut self, mut row: Vec<Tree>) {
        calculate_ew_visibilities(row.iter_mut(), &mut self.visible_count);
        calculate_ew_visibilities(row.iter_mut().rev(), &mut self.visible_count);
        self.terrain.push_row(row);
    }
    pub fn calculate_ns_visibilities(&mut self) {
        let width = self.terrain.width();
        calculate_ns_visibilities(self.terrain.iter_mut(), width, &mut self.visible_count);
        calculate_ns_visibilities(
            self.terrain.iter_mut().rev(),
            width,
            &mut self.visible_count,
        );
    }
    pub fn calculate_scenic_scores(&mut self) {
        for pt in self.terrain.points() {
            let scenic_score = self.scenic_score(pt);
            self.terrain[pt].scenic_score = scenic_score;
            if self.terrain[pt].visible {
                if scenic_score > self.max_visible_score {
                    self.max_visible_score = scenic_score;
                }
            } else {
                if scenic_score > self.max_hidden_score {
                    self.max_hidden_score = scenic_score;
                }
            }
        }
    }
    fn scenic_score(&self, pt: Point2<usize>) -> usize {
        let height = self.terrain[pt].height;

        let view_north = viewing_distance((0..pt.1).rev(), |ii| {
            self.terrain[pt.with_y(ii)].height < height
        });
        let view_south = viewing_distance((pt.1 + 1)..self.terrain.height(), |ii| {
            self.terrain[pt.with_y(ii)].height < height
        });

        let view_west = viewing_distance((0..pt.0).rev(), |jj| {
            self.terrain[pt.with_x(jj)].height < height
        });
        let view_east = viewing_distance((pt.0 + 1)..self.terrain.width(), |jj| {
            self.terrain[pt.with_x(jj)].height < height
        });

        view_north * view_south * view_west * view_east
    }
    pub fn dump(&self) {
        for row in self.terrain.iter() {
            let mut visible: bool = true;
            print!("\x1B[38;5;10m");
            for tree in row {
                if visible != tree.visible {
                    visible = tree.visible;
                    print!("\x1B[38;5;{}m", if visible { "10" } else { "3" });
                }
                print!("{}", tree.height - 1);
            }
            println!("\x1B[m");
        }
    }
    pub fn dump_extended(&self) {
        for row in self.terrain.iter() {
            for tree in row {
                let visibility: u8 = if tree.visible { 255 } else { 128 };
                let max_score = if tree.visible {
                    self.max_visible_score
                } else {
                    self.max_hidden_score
                };
                let scenic_quality: u8 = (tree.scenic_score * 255 / max_score) as u8;
                let high_byte = scale_u8(128 + scale_u8(scenic_quality, 127), visibility);
                let low_byte = scale_u8(128 - scale_u8(scenic_quality, 128), visibility);
                print!(
                    "\x1B[38;2;{};{};{}m{}",
                    if tree.visible { low_byte } else { high_byte },
                    high_byte,
                    low_byte,
                    char::from(tree),
                );
            }
            println!("\x1B[m");
        }
    }
}

fn viewing_distance<T>(
    range: impl Iterator<Item = T>,
    mut visibility: impl FnMut(T) -> bool,
) -> usize {
    let mut distance: usize = 0;
    for item in range {
        distance += 1;
        if !visibility(item) {
            break;
        }
    }

    distance
}

fn calculate_ew_visibilities<'a, 'b>(
    row: impl Iterator<Item = &'a mut Tree>,
    visible_count: &'b mut usize,
) {
    let mut max_height: u8 = 0;
    for tree in row {
        if !tree.update_visibility(&mut max_height, visible_count) {
            break;
        }
    }
}

fn calculate_ns_visibilities<'a, 'b>(
    map: impl Iterator<Item = &'a mut Vec<Tree>>,
    width: usize,
    visible_count: &'b mut usize,
) {
    let mut max_heights: Vec<u8> = vec![0; width];
    for row in map {
        for (tree, max_height) in row.iter_mut().zip(max_heights.iter_mut()) {
            tree.update_visibility(max_height, visible_count);
        }
    }
}
