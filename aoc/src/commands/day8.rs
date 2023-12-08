use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use std::cmp::Ordering;

use intersection::hash_set;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alphanumeric1, digit1, newline, space0, space1, u32},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
pub struct Day8 {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_range(input: &str) -> IResult<&str, InclusiveRange> {
    let (input, soil_map) = separated_list1(space1, u32)(input)?;
    let offset: usize = soil_map[0] as usize;
    let range: InclusiveRange =
        InclusiveRange::new(soil_map[1] as usize, soil_map[2] as usize, offset);
    Ok((input, range))
}

fn parse_map(input: &str) -> IResult<&str, Vec<InclusiveRange>> {
    let (input, soil_maps) = separated_list1(newline, parse_range)(input)?;
    Ok((input, soil_maps))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space1(input)?;
    let (input, mut seeds) = separated_list1(space1, u32)(input)?;
    let coalesced_seeds: Vec<InclusiveRange> = Vec::new();
    Ok((input, seeds))
}

pub fn keep(
    source: &Vec<InclusiveRange>,
    mymap: (usize, Vec<InclusiveRange>),
) -> Vec<InclusiveRange> {
    todo!()
}

pub fn get_seeds(seeds: Vec<u32>) -> Vec<InclusiveRange> {
    let mut i = 0;
    let mut newseeds: Vec<InclusiveRange> = Vec::new();
    while i < seeds.len() - 1 {
        let start: usize = seeds[i] as usize;
        let length: usize = seeds[i + 1] as usize;
        let range: InclusiveRange = InclusiveRange::new(start, length, 0);
        newseeds.push(range);
        i += 2;
    }
    newseeds.sort();
    let mut coalesced_seeds: Vec<InclusiveRange> = Vec::new();
    while !newseeds.is_empty() {
        let mut top = newseeds.pop().unwrap();
        loop {
            if !newseeds.is_empty() && top.overlaps(&newseeds[0]) {
                let newtop = newseeds.pop().unwrap();
                top = top.coalesce(&newtop).unwrap();
            } else {
                coalesced_seeds.push(top);
                break;
            }
        }
    }
    coalesced_seeds
}

fn parse_almanac2(input: &str) -> IResult<&str, usize> {
    let (input, seeds) = parse_seeds(input)?;
    let mut seeds: Vec<InclusiveRange> = get_seeds(seeds);

    let tags: Vec<String> = vec![
        "seed-to-soil map:".to_owned(),
        "soil-to-fertilizer map:".to_owned(),
        "fertilizer-to-water map:".to_owned(),
        "water-to-light map:".to_owned(),
        "light-to-temperature map:".to_owned(),
        "temperature-to-humidity map:".to_owned(),
        "humidity-to-location map:".to_owned(),
    ];

    let mut maps: Vec<Vec<InclusiveRange>> = vec![];
    for mytag in tags.iter() {
        let (input, _) = many0(newline)(input)?;
        let (input, _) = take_until(":")(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = many0(newline)(input)?;
        let (input, mut seed_to_soil) = parse_map(input)?;
        let (input, _) = many0(newline)(input)?;
        println!("parse {mytag:?}");
        maps.push(seed_to_soil);
    }

    for mapping in maps {}

    let min: usize = 0;

    Ok((input, min))
}

#[derive(Debug, Default, Eq)]
pub struct InclusiveRange {
    start: usize,
    end: usize,
    projection: Projection,
}

enum Projection {
    Add(usize),
    Subtract(usize),
    None,
}

impl Default for Projection {
    fn default() -> Self {
        Projection::None
    }
}

impl InclusiveRange {
    fn new(start: usize, length: usize) -> Self {
        Self { start, end: start + length, ..Default::default() }
    }

    pub fn set_addition(&mut self, projection: usize) -> &mut Self {
        self.projection = Projection::Add(projection);
        self
    }

    pub fn set_subtraction(&mut self, projection: usize) -> &mut Self {
        self.projection = Projection::Subtract(projection);
        self
    }

    pub fn project(&mut self) -> &mut Self {
        match self.projection {
            Projection::Add(x) => {
                self.start += x;
                self.end += x;
            }
            Projection::Subtract(x) => {
                self.start -= x;
                self.end -= x;
            }
            Projection::None => {}
        };
        self.projection = Projection::None;
        self
    }

    // check if there is an overlap between self and other
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    // return portion of self that doesn't overlap with other
    pub fn lower_disjunction(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) && self.start < other.start {
            Some(InclusiveRange::new(self.start, other.start))
        } else {
            None
        }
    }

    // return portion of self that doesn't overlap with other
    pub fn upper_disjunction(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) && self.end > other.end {
            Some(InclusiveRange::new(other.end + 1, self.end))
        } else {
            None
        }
    }

    // map intersecting portion of other
    pub fn conjunction(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            let start: usize = if self.start >= other.start { self.start } else { other.start };
            let end: usize = if self.end <= other.end { self.end } else { other.end };
            let mut conjunction: InclusiveRange = InclusiveRange::new(start, end);
            let &mut conjunction = match self.projection {
                Projection::Add(x) => conjunction.set_addition(x).project(),
                Projection::Subtract(x) => conjunction.set_subtraction(x).project(),
                Projection::None => &mut conjunction,
            };
            Some(conjunction)
        } else {
            None
        }
    }
}

impl Ord for InclusiveRange {
    fn cmp(&self, other: &Self) -> Ordering {
        other.start.cmp(&self.start)
    }
}

impl PartialOrd for InclusiveRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InclusiveRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl CommandImpl for Day8 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (input, almanac) = parse_almanac2(&string).unwrap();
        println!("min: {almanac}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let x: String = "seeds: 79 14 55 13".to_string();
        let input: &str = &x;
        let (input, actual) = parse_seeds(input).unwrap();
        let expected: Vec<u32> = vec![79, 14, 55, 13];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_soilmap() {
        let x: String = "0 15 37
37 52 2
"
        .to_string();
        let input: &str = &x;
        let (input, actual) = parse_map(input).unwrap();
        let expected: Vec<(usize, InclusiveRange)> =
            vec![(0, InclusiveRange::new(15, 37)), (37, InclusiveRange::new(52, 2))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_range_contains() {
        let this: InclusiveRange = InclusiveRange::new(5, 5);
        let that: InclusiveRange = InclusiveRange::new(6, 3);
        assert!(this.contains(&that));
    }

    #[test]
    fn test_range_overlaps() {
        let this: InclusiveRange = InclusiveRange::new(5, 5);
        let that: InclusiveRange = InclusiveRange::new(9, 1);
        assert!(this.overlaps(&that));
    }

    #[test]
    fn test_range_overlap() {
        let this: InclusiveRange = InclusiveRange::new(5, 5);
        let temp: InclusiveRange = InclusiveRange::new(5, 5);
        let that: InclusiveRange = this.overlap(&temp).unwrap();
        assert_eq!(this, that);
    }

    #[test]
    fn test_range_disjunction() {
        let this: InclusiveRange = InclusiveRange::new(5, 5);
        let temp: InclusiveRange = InclusiveRange::new(5, 5);
        let that = this.disjunction(&temp);
        assert!(that.is_none());

        let this: InclusiveRange = InclusiveRange::new(4, 5);
        let temp: InclusiveRange = InclusiveRange::new(5, 5);
        let that = this.disjunction(&temp).unwrap();
        let expected: InclusiveRange = InclusiveRange::new(4, 1);
        assert_eq!(that.len(), 1);
        assert_eq!(expected, that[0]);
    }
}
