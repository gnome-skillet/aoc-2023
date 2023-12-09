use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use intersection::hash_set;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, i32, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Parser, Debug)]
pub struct Day9 {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, sequence) = separated_list1(space1, i32)(input)?;
    Ok((input, sequence))
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, sequences) = separated_list1(newline, parse_sequence)(input)?;

    Ok((input, sequences))
}

pub fn differences(slice: &[i32]) -> Vec<i32> {
    let diffs: Vec<i32> = slice.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    diffs
}

pub fn get_differences(sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut curr: Vec<i32> = sequence.to_vec();
    loop {
        let mut sum: i32 = 0;
        for s in curr.iter() {
            sum += s;
        }
        let diff: Vec<i32> = differences(&curr);
        diffs.push(curr);
        if sum == 0 {
            break;
        }
        curr = diff;
    }
    diffs
}

pub fn evaluate_rhs(sequences: Vec<Vec<i32>>) -> i32 {
    let mut carry_over: i32 = 0i32;
    let mut i: usize = sequences.len();
    let mut vecs: Vec<i32> = Vec::new();
    while i != 0 {
        carry_over = sequences[i - 1].last().unwrap() + carry_over;
        vecs.push(carry_over);
        i -= 1;
    }
    let last_value: i32 = *vecs.last().unwrap();
    last_value
}

pub fn evaluate_lhs(sequences: Vec<Vec<i32>>) -> i32 {
    let mut carry_over: i32 = 0i32;
    let mut i: usize = sequences.len();
    let mut vecs: Vec<i32> = Vec::new();
    while i != 0 {
        carry_over = sequences[i - 1][0] - carry_over;
        vecs.push(carry_over);
        i -= 1;
    }
    let last_value: i32 = *vecs.last().unwrap();
    last_value
}

pub fn solve_parta(sequences: Vec<Vec<i32>>) -> i32 {
    let mut results: Vec<i32> = Vec::new();
    for seq in sequences.iter() {
        let differences: Vec<Vec<i32>> = get_differences(&seq);
        //println!("differences: {differences:#?}");
        let evaluation: i32 = evaluate_rhs(differences);
        results.push(evaluation);
        println!("evaluation: {evaluation:#?}");
    }
    results.iter().sum()
}

pub fn solve_partb(sequences: Vec<Vec<i32>>) -> i32 {
    let mut results: Vec<i32> = Vec::new();
    for seq in sequences.iter() {
        let differences: Vec<Vec<i32>> = get_differences(&seq);
        //println!("differences: {differences:#?}");
        let evaluation: i32 = evaluate_lhs(differences);
        results.push(evaluation);
        println!("evaluation: {evaluation:#?}");
    }
    results.iter().sum()
}

impl CommandImpl for Day9 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (_, sequences) = parse_sequences(&string).unwrap();
        //println!("sequences {sequences:#?}");
        //let solution: i32 = solve_parta(sequences);
        //println!("solution a: {solution:#?}");
        let solution: i32 = solve_partb(sequences);
        println!("solution b: {solution:#?}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_overlaps() {
        let this: Vec<i32> = vec![1, 2, 3, 5, 8, 13];
        let expected: Vec<i32> = vec![1, 1, 2, 3, 5];
        let actual: Vec<i32> = differences(&this);
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), this.len() - 1);
    }
}
