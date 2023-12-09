use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use intersection::hash_set;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Parser, Debug)]
pub struct Day8 {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, v) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    Ok((input, (v.0, v.1)))
}

fn parse_lookup(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, source) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, left_right) = parse_tuple(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (source, left_right)))
}

fn parse_lookups(input: &str) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    let (input, lookups) = separated_list1(newline, parse_lookup)(input)?;
    Ok((input, lookups))
}

fn parse_router(input: &str) -> IResult<&str, (&str, Router)> {
    let (input, steps) = alphanumeric1(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, lookups) = parse_lookups(input)?;
    let mapping: Router = Router::new(lookups);

    Ok((input, (steps, mapping)))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        (max, min) = (min, res);
        //max = min;
        //min = res;
    }
}

pub struct Router<'a> {
    route: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Router<'a> {
    pub fn new(router: Vec<(&'a str, (&'a str, &'a str))>) -> Self {
        let mut route: HashMap<&str, (&str, &str)> = HashMap::new();
        for step in router {
            route.insert(step.0, step.1);
        }
        Router { route }
    }

    pub fn traverse(&self, origin: &str, destination: char, path: &str) -> usize {
        let mut nsteps: usize = 0;
        let mut location: &str = origin;
        for c in path.chars().cycle() {
            if location.ends_with(destination) {
                break;
            }
            nsteps += 1;
            let fork: &(&str, &str) = self.route.get(location).unwrap();
            location = if c == 'L' { fork.0 } else { fork.1 };
        }
        nsteps
    }

    pub fn traverse_paths(&self, path: &str, origin: char, destination: char) -> Option<usize> {
        //let mut steps = path.chars().cycle();
        let mut locations: Vec<&str> = Vec::new();
        for s in self.route.keys() {
            if s.ends_with(origin) {
                locations.push(s);
            }
        }
        let mut distances: Vec<usize> = Vec::new();
        for location in locations.iter() {
            let distance: usize = self.traverse(location, 'Z', path);
            distances.push(distance);
        }

        distances.into_iter().reduce(lcm)
    }
}

impl CommandImpl for Day8 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (_, (path, router)) = parse_router(&string).unwrap();
        if let Some(distance) = router.traverse_paths(path, 'A', 'Z') {
            println!("distance {distance:#?}");
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
