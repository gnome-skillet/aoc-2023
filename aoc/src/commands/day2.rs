use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use serde::{Deserialize, Serialize};

use std::fs::File;

use csv::{ReaderBuilder,Error};

#[derive(Parser, Debug)]
pub struct Day2 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    direction: Direction,
    displacement: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Default, Debug)]
pub struct SubmarinePosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl SubmarinePosition {
    fn execute(&mut self, command: Command) {
        match command {
            Command { direction: Direction::Forward, displacement } => {
                self.horizontal += displacement;
                self.depth += self.aim * displacement;
            },
            Command { direction: Direction::Up, displacement } => {
                self.aim -= displacement;
            },
            Command { direction: Direction::Down, displacement } => {
                self.aim += displacement;
            },
            _ => {
                todo!();
            }
        }
    }

    fn multiply(self) -> i32 {
        self.horizontal * self.depth
    }
}

impl CommandImpl for Day2 {
    fn main(&self) -> Result<(), DynError> {
        let mut submarine_position: SubmarinePosition = Default::default();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b' ')
            .has_headers(false)
            .from_path(&self.input)?;
        let iter = rdr.deserialize();

        for record in iter {
            let command: Command = record?; 
            submarine_position.execute(command);
        }
        println!("submarine position: {:?}", submarine_position);
        println!("submarine value: {:?}", submarine_position.multiply());
        Ok(())
    }
}
