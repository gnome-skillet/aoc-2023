use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use intersection::hash_set;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alphanumeric1, digit1, newline, space0, space1, u32},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
pub struct Day4 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, PartialEq)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    pub fn new(number: u32, color: &str) -> Cube {
        match color {
            "red" => Cube::Red(number),
            "green" => Cube::Green(number),
            "blue" => Cube::Blue(number),
            _ => panic!(),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Bag {
    colored_cubes: HashMap<String, u32>,
}

impl Bag {
    pub fn new(cubes: Vec<Cube>) -> Self {
        let mut colored_cubes: HashMap<String, u32> = HashMap::new();
        for cube in cubes {
            match cube {
                Cube::Red(x) => {
                    colored_cubes.insert("red".to_string(), x);
                }
                Cube::Blue(x) => {
                    colored_cubes.insert("blue".to_string(), x);
                }
                Cube::Green(x) => {
                    colored_cubes.insert("green".to_string(), x);
                }
            }
        }
        Bag { colored_cubes }
    }

    pub fn combine(&mut self, other: &Bag) -> &mut Self {
        for (color, count) in other.colored_cubes.iter() {
            if self.colored_cubes.contains_key(&*color) {
                let this_count = self.colored_cubes.get(&*color).unwrap();
                if count <= this_count {
                    continue;
                }
            }
            self.colored_cubes.insert(color.to_string(), *count);
        }
        self
    }

    pub fn powerset(&self) -> u32 {
        self.colored_cubes.values().product()
    }
}

fn parse_game(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_numbers) = separated_list1(space1, u32)(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, numbers) = separated_list1(space1, u32)(input)?;
    let numbers: HashSet<u32> = HashSet::from_iter(numbers.iter().cloned());
    let winning_numbers: HashSet<u32> = HashSet::from_iter(winning_numbers.iter().cloned());
    let score = hash_set::intersection(vec![winning_numbers, numbers]);
    let nmatches: u32 = score.len() as u32;

    let updated_score: u32 = if nmatches == 0 { 0u32 } else { 2u32.pow(nmatches - 1) };
    Ok((input, updated_score))
}

fn parse_games(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;
    Ok((input, games))
}

impl CommandImpl for Day4 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (input, games) = parse_games(&string).unwrap();
        println!("games: {games:?}");
        let score: u32 = games.iter().sum();
        println!("score: {score}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube() {
        let x: String = "6 green".to_string();
        let input: &str = &x;
        let (input, actual) = parse_cube(input).unwrap();
        let expected: Cube = Cube::new(6u32, "green");
        assert_eq!(actual, expected);
        let x: String = "4 red".to_string();
        let input: &str = &x;
        let (input, actual) = parse_cube(input).unwrap();
        let expected: Cube = Cube::new(4u32, "red");
        assert_eq!(actual, expected);
    }
}
