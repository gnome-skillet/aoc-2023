use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alphanumeric1, digit1, newline, space0, space1, u32},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
pub struct Day2 {
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
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Bag {
    colored_cubes: HashMap<String, u32>,
}

impl Bag {
    pub fn new(cubes: Vec<Cube>) -> Self {
    }
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, number) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((tag("green"), tag("red"), tag("blue")))(input)?;
    Ok((input, Cube::new(number, color)))
}

fn parse_cubes(input: &str) -> IResult<&str, Bag> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    let bag: Bag = Bag::new(cubes);
    Ok((input, bag))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Bag>> {
    let (input, _) = tag("Game ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, game) = separated_list1(tag("; "), parse_cubes)(input)?;
    Ok((input, game))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Vec<Bag>>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;
    Ok((input, games))
}

impl CommandImpl for Day2 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (input, games) = parse_games(&string).unwrap();
        let cubes: Vec<Cube> = vec![Cube::Red(12), Cube::Blue(14), Cube::Green(13)];
        let bag: Bag = Bag::new(cubes);
        println!("{bag:#?}");
        let mut vec: Vec<u32> = Vec::new();
        for (i, game) in games.iter().enumerate() {
            let mut doit: bool = true;
            for draw in game {
                if !bag.contains(&draw) {
                    println!("uncontained ({i}): {draw:#?}");
                    doit = false;
                    break;
                } else {
                    println!("contained ({i}): {draw:#?}");
                }
            }
            if doit {
                vec.push(i as u32 + 1);
            }
        }

        let mut powersets: Vec<u32> = Vec::new();
        for game in games.iter() {
            let mut default_bag: Bag = Bag::default();
            for bag in game {
                default_bag.combine(bag);
            }
            let p: u32 = default_bag.powerset();
            powersets.push(p);
        }
        let sum: u32 = vec.iter().sum();
        let powersum: u32 = powersets.iter().sum();
        println!("vec: {vec:?}");
        println!("sum: {sum}");
        println!("powersets: {powersets:?}");
        println!("powersum: {powersum}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cubes() {
        let x: String = "6 green, 4 red".to_string();
        let input: &str = &x;
        let (input, actual) = parse_cubes(input).unwrap();
        let green: Cube = Cube::new(6u32, "green");
        let red: Cube = Cube::new(4u32, "red");
        let expected: Bag = Bag::new(vec![green, red]);
        assert_eq!(actual, expected);
    }
}
