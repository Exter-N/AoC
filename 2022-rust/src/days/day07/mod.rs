use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u32};
use nom::combinator::{map, rest};
use nom::sequence::{preceded, separated_pair};

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

mod fs;
mod state;

use state::Session;

#[derive(Default)]
struct Day7 {
    verbose: bool,
    state: Session,
}

impl Day7 {
    fn new(verbose: bool) -> Self {
        Self {
            verbose,
            ..Default::default()
        }
    }
}

enum SessionLine<'a> {
    MoveToRootCommand,
    MoveToParentCommand,
    MoveToChildCommand { name: &'a str },
    ListCommand,
    DirectoryEntry { name: &'a str },
    FileEntry { size: u32, name: &'a str },
}

impl<'a> SessionLine<'a> {
    fn update_state(self, state: &mut Session) -> Result<(), Box<dyn Error>> {
        Ok(match self {
            Self::MoveToRootCommand => state.move_to_root(),
            Self::MoveToParentCommand => state.move_to_parent()?,
            Self::MoveToChildCommand { name } => state.move_to_child(&name)?,
            Self::ListCommand => (),
            Self::DirectoryEntry { name } => {
                state.assert_directory(&name)?;
            }
            Self::FileEntry { size, name } => state.assert_file(&name, size)?,
        })
    }
}

impl LineStreamHandler for Day7 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let session_line = parse_full_string(
            line,
            alt((
                preceded(
                    tag("$ "),
                    alt((
                        preceded(
                            tag("cd "),
                            alt((
                                map(char('/'), |_| SessionLine::MoveToRootCommand),
                                map(tag(".."), |_| SessionLine::MoveToParentCommand),
                                map(rest, |name: &str| SessionLine::MoveToChildCommand { name }),
                            )),
                        ),
                        map(tag("ls"), |_| SessionLine::ListCommand),
                    )),
                ),
                map(preceded(tag("dir "), rest), |name: &str| {
                    SessionLine::DirectoryEntry { name }
                }),
                map(
                    separated_pair(u32, char(' '), rest),
                    |(size, name): (u32, &str)| SessionLine::FileEntry { size, name },
                ),
            )),
        )?;

        session_line.update_state(&mut self.state)?;

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Small dirs total size: {}",
            SILVER_ANSI,
            self.state.small_dirs_total(100_000)
        );
        println!(
            "[{}] Dir to delete size:    {}",
            GOLD_ANSI,
            self.state.to_delete_size(70_000_000 - 30_000_000)
        );
        if self.verbose {
            self.state.dump_fs();
        }

        Ok(())
    }
}

pub fn new(
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((7, "No Space Left On Device", Box::new(Day7::new(verbose))))
}
