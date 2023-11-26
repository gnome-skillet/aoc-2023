use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use std::collections::HashMap;

use std::ops::AddAssign;
use std::ops::Add;
use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, i32},
    multi::{separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Parser, Debug)]
pub struct Day5 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug,Copy,Clone)]
pub struct Line {
    from_point: Point,
    to_point: Point,
    index: Point,
}

impl Line {
    pub fn new(from_point: Point, to_point: Point) -> Self {
        let (from, to) = if from_point <= to_point {
            (from_point, to_point)
        } else {
            (to_point, from_point)
        };
        Line {
            from_point: from,
            to_point: to,
            index: from,
        }
    }

    pub fn iter(&self) -> LineIterator {
        LineIterator {
            line: *self,
            index: self.from_point,
        }
    }

    pub fn incrementor(&self) -> Option<Point> {
        if self.is_vertical() {
            Some(Point { x: 0i32, y: 1i32})
        } else if self.is_horizontal() {
            Some(Point { x: 1i32, y: 0i32})
        } else if self.is_diagonal() && self.positive_slope() {
            Some(Point { x: 1i32, y: 1i32})
        } else if self.is_diagonal() && !self.positive_slope() {
            Some(Point { x: 1i32, y: -1i32})
        } else {
            None
        }
    }

    pub fn from_point(&self) -> &Point {
        &self.from_point
    }

    pub fn to_point(&self) -> &Point {
        &self.to_point
    }

    pub fn is_horizontal(&self) -> bool {
        self.from_point.y == self.to_point.y
    }

    pub fn is_vertical(&self) -> bool {
        self.from_point.x == self.to_point.x
    }

    pub fn is_diagonal(&self) -> bool {
        let deltax: i32 = self.to_point.x - self.from_point.x;
        let deltay: i32 = (self.to_point.y - self.from_point.y);
        deltax == deltay.abs()
    }

    pub fn positive_slope(&self) -> bool {
        self.to_point.y > self.from_point.y
    }
}

pub struct LineIterator {
    line: Line,
    index: Point,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= self.line.to_point {
            let result = Some(self.index);
            if let Some(step) = self.line.incrementor().as_mut() {
                self.index += *step;
            }
            result
        } else {
            None
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {

        if self.index <= self.to_point {
            let current = Some(self.index);
            if let Some(step) = self.incrementor() {
                self.index += step;
            }
            current
        } else {
            None
        }
    }
}

#[derive(Eq, Hash, Debug, Clone, Copy, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(p: (i32,i32)) -> Self {
        Point { x: p.0, y: p.1 }
    }

    pub fn incrementor(&self, other: &Self) -> Option<Self> {
        let matchx: i32 = self.x;
        let matchy: i32 = self.y;
        match other {
            Point { x: matchx, y } => Some(Point { x: 0i32, y: 1i32}),
            Point { x, y: matchy } => Some(Point { x: 1i32, y: 0i32}),
            _ => None,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl From<(i32,i32)> for Point {
    fn from(p: (i32,i32)) -> Self {
        Point::new(p)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn le(&self, other: &Self) -> bool {
        self.x < other.x || (self.x == other.x && self.y <= other.y) 
    }

    fn lt(&self, other: &Self) -> bool {
        self.x < other.x || (self.x == other.x && self.y < other.y) 
    }

    fn gt(&self, other: &Self) -> bool {
        self.x > other.x || (self.x == other.x && self.y > other.y) 
    }

    fn ge(&self, other: &Self) -> bool {
        self.x > other.x || (self.x == other.x && self.y >= other.y) 
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Grid {
    points: HashMap<Point, i32>,
}

impl Grid {
    pub fn new() -> Self {
        let points: HashMap<Point,i32> = HashMap::new();
        Grid { points }
    }

    pub fn add_point(&mut self, p: &Point) {
        *self.points.entry(*p).or_insert(0) += 1;
    }

    pub fn add_line(&mut self, l: Line) {
        if l.is_vertical() || l.is_horizontal() || l.is_diagonal() {
            for p in l.iter() {
                self.add_point(&p);
            }
        }
    }

    pub fn overlap_tally(&self) -> usize {
        self.points
            .values()
            .filter(|x| *x > &1)
            .count()
    }
}

fn parse_tuple(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, tuple) = separated_pair(i32, tag(","), i32)(input)?;
    Ok((input, tuple))
}

fn parse_arrow(input: &str) -> IResult<&str, &str> {
    let (i, arrow) = delimited(tag(" "), tag("->"), tag(" "))(input)?;
    Ok((i, arrow))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, _) = space0(input)?;
    let (input, from_point) = parse_tuple(input)?;
    let (input, _arrow) = parse_arrow(input)?;
    let (input, to_point) = parse_tuple(input)?;
    let line: Line = Line::new(Point::from(from_point), Point::from(to_point));
    Ok((input, line))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (i, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((i, lines))
}

impl CommandImpl for Day5 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input)?;
        let input: &str = &string;
        //let (_, lines) = parse_lines(input).unwrap();
        let (_i, lines) = parse_lines(input).unwrap();
        let mut grid: Grid = Grid::new();
        for line in lines {
            grid.add_line(line);
        }
        println!("there are {:?} points > 2", grid.overlap_tally());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line_vertical() {
        let x: &str = "3,1 -> 0,1";
        let (_, line) = parse_line(x).unwrap();
        assert!(line.is_horizontal());
        let expected_from: Point = Point::new((0i32,1i32));
        let expected_to: Point = Point::new((3i32,1i32));
        assert_eq!(line.from_point, expected_from,"from should be smaller than to");
        assert_eq!(line.to_point, expected_to, "to should be larger than from");
        for (i, p) in line.iter().enumerate() {
            let expected: Point = Point::new((i as i32, 1i32));
            assert_eq!(p, expected, "vertical increment expect step({i}): x {p:?} to increase");
        }
    }

    #[test]
    fn test_parse_line_horizontal() {
        let x: &str = "1,1 -> 2,2";
        let (_, line) = parse_line(x).unwrap();
        let expected_from: Point = Point::new((1i32,1i32));
        let expected_to: Point = Point::new((2i32,2i32));
        assert_eq!(line.from_point, expected_from);
        assert_eq!(line.to_point, expected_to);
    }

    #[test]
    fn test_line_iterator() {
        let x: &str = "1,1 -> 1,3";
        let (_, mut line) = parse_line(x).unwrap();
        if let Some(p) = line.next() {
            let expected: Point = Point::new((1i32, 1i32));
            assert_eq!(p, expected);
        }
        if let Some(p) = line.next() {
            let expected: Point = Point::new((1i32, 2i32));
            assert_eq!(p, expected);
        }
        if let Some(p) = line.next() {
            let expected: Point = Point::new((1i32, 3i32));
            assert_eq!(p, expected);
        }
        let x = line.next();
        assert_eq!(x, None);
    }

    #[test]
    fn test_parse_lines() {
        let x: &str = "0,0 -> 1,1
2,2 -> 3,3";
        let (_, lines) = parse_lines(x).unwrap();
        let expected_from: Point = Point::new((0i32,0i32));
        let expected_to: Point = Point::new((1i32,1i32));
        assert_eq!(lines[0].from_point(), &expected_from);
        assert_eq!(lines[0].to_point(), &expected_to);
        let expected_from: Point = Point::new((2i32,2i32));
        let expected_to: Point = Point::new((3i32,3i32));
        assert_eq!(lines[1].from_point(), &expected_from);
        assert_eq!(lines[1].to_point(), &expected_to);
    }

    #[test]
    fn test_parse_line_is_horizontal() {
        let x: &str = "0,9 -> 5,9";
        let (_, lines) = parse_lines(x).unwrap();
        assert!(lines[0].is_horizontal());
        assert!(!lines[0].is_vertical());
        let x: &str = "5,9 -> 0,9";
        let (_, lines) = parse_lines(x).unwrap();
        assert!(lines[0].is_horizontal());
        assert!(!lines[0].is_vertical());
     }

    #[test]
    fn test_parse_line_is_vertical() {
        let x: &str = "9,0 -> 9,5";
        let (_, lines) = parse_lines(x).unwrap();
        assert!(lines[0].is_vertical());
        assert!(!lines[0].is_horizontal());
        let x: &str = "9,5 -> 9,0";
        let (_, lines) = parse_lines(x).unwrap();
        let expected_from: Point = Point::new((9i32, 0i32));  
        assert!(lines[0].is_vertical());
        assert!(!lines[0].is_horizontal());
        assert_eq!(lines[0].from_point, expected_from);
     }

    #[test]
    fn test_parse_lines_fullinput() {
        let x: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let (_, lines) = parse_lines(x).unwrap();
        let expected_from: Point = Point::new((0i32,9i32));
        let expected_to: Point = Point::new((5i32,9i32));
        assert_eq!(lines[0].from_point(), &expected_from);
        assert_eq!(lines[0].to_point(), &expected_to);
    }
}
