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
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
pub struct Day5 {
    #[clap(long, short)]
    input: PathBuf,
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

    Ok((input, nmatches))
}

fn parse_range(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, soil_map) = separated_list1(space1, u32)(input)?;
    Ok((input, soil_map))
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, soil_maps) = separated_list1(newline, parse_range)(input)?;
    Ok((input, soil_maps))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space1(input)?;
    let (input, seeds) = separated_list1(space1, u32)(input)?;
    Ok((input, seeds))
}

pub fn keep(source: &Vec<InclusiveRange>, mymap: (usize,Vec<InclusiveRange>)) -> Vec<InclusiveRange> {
    let mut destination: HashSet<u32> = HashSet::new();
    let mut source_found: HashSet<u32> = HashSet::new();

    for m in mymap {
        let dest_start: u32 = m[0];
        let source_start: u32 = m[1]; 
        for s in source.iter() {
            if *s >= source_start && *s - source_start < m[2] {
                let i: u32 = *s - source_start;
                let dest_no: u32 = dest_start + i as u32;
                let source_no: u32 = source_start + i as u32;
                destination.insert(dest_no);
                source_found.insert(source_no);
            }
        }
    }

        for s in source.iter() {
            if source_found.contains(s) {
            } else {
                destination.insert(*s);
            }
        }
    destination
}

fn parse_almanac(input: &str) -> IResult<&str, u32> {
    let (input, seeds) = parse_seeds(input)?;
    let seeds: HashSet<u32> = HashSet::from_iter(seeds.iter().cloned());

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("seed-to-soil map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, seed_to_soil) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&seeds,seed_to_soil);

    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, soil_to_fertilizer) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,soil_to_fertilizer);

    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, fertilizer_to_water) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,fertilizer_to_water);

    let seed_to_soil: HashMap<u32,u32> = HashMap::new();

    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, water_to_light) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,water_to_light);

    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, light_to_temp) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,light_to_temp);

    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, temp_to_humidity) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,temp_to_humidity);

    let (input, _) = tag("humidity-to-location map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, humidity_to_location) = parse_map(input)?;
    let numbers: HashSet<u32> = keep(&numbers,humidity_to_location);
    let min = numbers.iter().min().unwrap();

    Ok((input, *min))
}

pub fn get_seeds(seeds: Vec<u32>) -> HashSet<u32> {
    let mut i = 0;
    let mut newseeds: HashSet<u32> = HashSet::new();
    while i < seeds.len() - 1 {
        let length: usize = seeds[i+1] as usize;
        for j in 0..length {
            let seed: u32 = seeds[i] + j as u32;
            newseeds.insert(seed);
        }
        i += 2;
    }
    newseeds
}

fn parse_almanac2(input: &str) -> IResult<&str, u32> {
    let (input, seeds) = parse_seeds(input)?;
    let seeds: HashSet<u32> = get_seeds(seeds); 

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("seed-to-soil map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, seed_to_soil) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&seeds,seed_to_soil);

    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, soil_to_fertilizer) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,soil_to_fertilizer);

    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, fertilizer_to_water) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,fertilizer_to_water);

    let seed_to_soil: HashMap<u32,u32> = HashMap::new();

    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, water_to_light) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,water_to_light);

    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, light_to_temp) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,light_to_temp);

    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, temp_to_humidity) = parse_map(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let numbers: HashSet<u32> = keep(&numbers,temp_to_humidity);

    let (input, _) = tag("humidity-to-location map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, humidity_to_location) = parse_map(input)?;
    let numbers: HashSet<u32> = keep(&numbers,humidity_to_location);
    let min = numbers.iter().min().unwrap();

    Ok((input, *min))
}

#[derive(Debug,Default,Eq)]
pub struct InclusiveRange {
    start: usize,
    length: usize,
}

impl InclusiveRange {
    pub fn new(start: usize, length: usize) -> Self {
        InclusiveRange {
            start: start,
            length: length,
        }
    }

    // check if there is an overlap between self and other
    pub fn overlaps(&self, other: &Self) -> bool {
        let end_of_self: usize = self.start + self.length - 1;
        let end_of_other: usize = other.start + other.length - 1;
        end_of_self >= other.start && end_of_other >= self.start
    }

    // check if self contains other 
    pub fn contains(&self, other: &Self) -> bool {
        let end_of_self: usize = self.start + self.length - 1;
        let end_of_other: usize = other.start + other.length - 1;
        self.start <= other.start && end_of_other <= end_of_self 
    }

    pub fn end(&self) -> usize {
        self.start + self.length - 1
    }

    // return portion of self that doesn't overlap with other
    pub fn disjunction(&self, other: &Self) -> Option<Vec<Self>> {
        if self.contains(other) && other.contains(self) {
            return None;
        }
        let mut nonoverlap: Vec<InclusiveRange> = Vec::new(); 
        if self.start < other.start {
            let newrange: InclusiveRange = InclusiveRange::new(self.start, other.start - self.start);
            nonoverlap.push(newrange);
        }
        if self.end() > other.end() {
            let newrange: InclusiveRange = InclusiveRange::new(other.end() + 1, self.end());
            nonoverlap.push(newrange);
        }
        Some(nonoverlap)
    }

    // return portion of self that overlaps with other
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.overlaps(other) {
            return None 
        }

        let end_of_self: usize = self.start + self.length - 1;
        let end_of_other: usize = other.start + other.length - 1;
        let start_of_overlap = if self.start >= other.start {
            self.start
        } else {
            other.start
        };
        let length_of_overlap = if end_of_self <= end_of_other {
            end_of_self - start_of_overlap + 1 
        } else {
            end_of_other - start_of_overlap + 1 
        };
        Some(InclusiveRange::new(start_of_overlap, length_of_overlap))
    }
}

impl Ord for InclusiveRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for InclusiveRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InclusiveRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.length == other.length
    }
}

impl CommandImpl for Day5 {
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
        let expected: Vec<u32> = vec![79,14, 55, 13]; 
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_soilmap() {
        let x: String = "0 15 37
37 52 2
39 0 15
".to_string();
        let input: &str = &x;
        let (input, actual) = parse_map(input).unwrap();
        let expected: Vec<Vec<u32>> = vec![vec![0,15,37], vec![37,52,2], vec![39,0,15]]; 
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_range_contains() {
        let this: InclusiveRange = InclusiveRange::new(5,5); 
        let that: InclusiveRange = InclusiveRange::new(6,3); 
        assert!(this.contains(&that));
    }

    #[test]
    fn test_range_overlaps() {
        let this: InclusiveRange = InclusiveRange::new(5,5); 
        let that: InclusiveRange = InclusiveRange::new(9,1); 
        assert!(this.overlaps(&that));
    }

    #[test]
    fn test_range_overlap() {
        let this: InclusiveRange = InclusiveRange::new(5,5); 
        let temp: InclusiveRange = InclusiveRange::new(5,5); 
        let that: InclusiveRange = this.overlap(&temp).unwrap(); 
        assert_eq!(this,that);
    }

    #[test]
    fn test_range_disjunction() {
        let this: InclusiveRange = InclusiveRange::new(5,5); 
        let temp: InclusiveRange = InclusiveRange::new(5,5); 
        let that = this.disjunction(&temp); 
        assert!(that.is_none());

        let this: InclusiveRange = InclusiveRange::new(4,5); 
        let temp: InclusiveRange = InclusiveRange::new(5,5); 
        let that = this.disjunction(&temp).unwrap(); 
        let expected: InclusiveRange = InclusiveRange::new(4,1); 
        assert_eq!(that.len(),1);
        assert_eq!(expected,that[0]);
    }
}
