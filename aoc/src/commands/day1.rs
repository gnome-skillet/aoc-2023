use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alphanumeric1, digit0, newline, space0, u32},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Parser, Debug)]
pub struct Day1 {
    #[clap(long, short)]
    input: PathBuf,
}

fn parse_number_string(input: &str) -> IResult<&str, u32> {
    println!("parse_number: {input:?}");
    let (input, line) = alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ))(input)?;
    let number: u32 = match line {
        "one" => 1u32,
        "two" => 2u32,
        "three" => 3u32,
        "four" => 4u32,
        "five" => 5u32,
        "six" => 6u32,
        "seven" => 7u32,
        "eight" => 8u32,
        "nine" => 9u32,
        "zero" => 0u32,
        _ => 0u32,
    };

    Ok((input, number))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    println!("parse_line({input:?})");
    //let (input, vec) = many1(preceded(alpha0, parse_number))(input)?;
    let (input, number) = preceded(alpha0, alt((u32, parse_number_string)))(input)?;
    //let (input, _) = alpha0(input)?;
    println!("number: {number:?}");
    Ok((input, number))
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    println!("parse_line({input:?})");
    //let (input, vec) = many1(preceded(alpha0, parse_number))(input)?;
    let (input, vec) = many1(parse_number)(input)?;
    //let (input, _) = alpha0(input)?;
    println!("vec: {vec:?}");
    Ok((input, vec))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    println!("parse_lines({input:?})");
    let (i, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((i, lines))
}

pub fn extract_numbers(string: &str) -> usize {
    let numbers =
        vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut vec: Vec<usize> = vec![];
    let mut index: usize = 0;
    while index < string.len() {
        let mut matched: bool = false;
        for (i, number) in numbers.iter().enumerate().skip(1) {
            let value_string = i.to_string();
            if string[index..].starts_with(number) {
                matched = true;
            } else if string[index..].starts_with(&value_string) {
                matched = true;
            }
            if matched {
                vec.push(i);
                index += 1;
                break;
            }
        }
        if !matched {
            index += 1;
        }
    }
    let result: usize = 10 * vec[0] + vec[vec.len() - 1];
    println!("extract_numbers({string:?}) = {result}");
    result
}

impl CommandImpl for Day1 {
    fn main(&self) -> Result<(), DynError> {
        let mut numbers: Vec<usize> = Vec::new();
        let file = File::open(&self.input)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let number: usize = extract_numbers(&line?);
            numbers.push(number);
        }
        let sum: usize = numbers.iter().sum();
        println!("the sum of {:?} numbers: is {sum}", numbers.len());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_string() {
        let x: String = "one".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 11usize;
        assert_eq!(actual, expected);
        let x: String = "two".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 22usize;
        let x: String = "three".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 33usize;
        assert_eq!(actual, expected);
        let x: String = "12345".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 15usize;
        assert_eq!(actual, expected);
        let x: String = "1three2".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 12usize;
        assert_eq!(actual, expected);
        let x: String = "four".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 44usize;
        assert_eq!(actual, expected);
        let x: String = "five".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 55usize;
        assert_eq!(actual, expected);
        let x: String = "six".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 66usize;
        assert_eq!(actual, expected);
        let x: String = "seven".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 77usize;
        assert_eq!(actual, expected);
        let x: String = "eight".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 88usize;
        assert_eq!(actual, expected);
        let x: String = "8".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 88usize;
        assert_eq!(actual, expected);
        let x: String = "nine".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 99usize;
        assert_eq!(actual, expected);
        let x: String = "9".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 99usize;
        assert_eq!(actual, expected);
        let x: String = "zsdfe9".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 99usize;
        assert_eq!(actual, expected);
        let x: String = "zsdfe9z".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 99usize;
        assert_eq!(actual, expected);
        let x: String = "onetwothreefourfivesixseveneightnine".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 19usize;
        assert_eq!(actual, expected);
        let x: String = "four77".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 47usize;
        assert_eq!(actual, expected);
        let x: String = "477".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 47usize;
        assert_eq!(actual, expected);
        let x: String = "47seven".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 47usize;
        assert_eq!(actual, expected);
        let x: String = "ckmb52fldxkseven3fkjgcbzmnr7".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 57usize;
        assert_eq!(actual, expected);
        let x: String = "gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 61usize;
        assert_eq!(actual, expected);
        let x: String = "2onetwocrgbqm7".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 27usize;
        assert_eq!(actual, expected);
        let x: String = "frkh2nineqmqxrvdsevenfive".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 25usize;
        assert_eq!(actual, expected);
        let x: String = "four9two".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 42usize;
        assert_eq!(actual, expected);
        let x: String = "5twomgkzsvg".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 52usize;
        assert_eq!(actual, expected);
        let x: String = "24".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 24usize;
        assert_eq!(actual, expected);

        let x: String = "pseven3threeeightseven".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 77usize;
        assert_eq!(actual, expected);

        let x: String = "8mgrxk".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 88usize;
        assert_eq!(actual, expected);

        let x: String = "ninefivetwojbhglxfxzfctwo8".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 98usize;
        assert_eq!(actual, expected);

        let x: String = "nin12345678ono".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 18usize;
        assert_eq!(actual, expected);

        let x: String = "foursix5".to_string();
        let input: &str = &x;
        let actual: usize = extract_numbers(input);
        let expected: usize = 45usize;
        assert_eq!(actual, expected);
    }
}
