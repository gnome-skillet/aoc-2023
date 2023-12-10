use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use intersection::hash_set;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, i32, newline, one_of, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Parser, Debug)]
pub struct Day10 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    None,
}

impl Pipe {
    pub fn new(c: char) -> Pipe {
        match c {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::None,
            _ => panic!(),
        }
    }

    pub fn is_corner(&self) -> bool {
        *self == Pipe::NorthEast || *self == Pipe::SouthWest
    }

    pub fn neighbors(&self, field: &Vec<Vec<Pipe>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut vec: Vec<(usize, usize)> = Vec::new();
        match *self {
            Pipe::Vertical | Pipe::Start | Pipe::NorthEast | Pipe::NorthWest => {
                if pos.0 > 0usize {
                    let upper_pipe: Pipe = field[pos.0 - 1][pos.1];
                    if connects_vertical(upper_pipe, *self) {
                        //println!("add left neighbor");
                        vec.push((pos.0 - 1, pos.1));
                    } else {
                        //println!("upper pipe incompatible with {self:#?}");
                    };
                } else {
                    //println!("skip upper pipe because {pos:#?}");
                };
            }
            _ => {}
        };
        match *self {
            Pipe::Vertical | Pipe::Start | Pipe::SouthEast | Pipe::SouthWest => {
                //println!("right vertical check");
                if pos.0 < field.len() - 1 {
                    let lower_pipe: Pipe = field[pos.0 + 1][pos.1];
                    if connects_vertical(*self, lower_pipe) {
                        //println!("add right neighbor");
                        vec.push((pos.0 + 1, pos.1));
                    } else {
                        //println!("lower pipe incompatible with {self:#?}");
                    };
                } else {
                    //println!("skip lower pipe below because {pos:#?}");
                };
            }
            _ => {}
        };

        match *self {
            Pipe::Horizontal | Pipe::Start | Pipe::NorthWest | Pipe::SouthWest => {
                //println!("upper horizontal check");
                if pos.1 > 0 {
                    let left_pipe: Pipe = field[pos.0][pos.1 - 1];
                    if connects_horizontal(left_pipe, *self) {
                        // println!("add lower neighbor");
                        vec.push((pos.0, pos.1 - 1));
                    } else {
                        //println!("left pipe incompatible with {self:#?}");
                    };
                } else {
                    //println!("skip left because {pos:#?}");
                };
            }
            _ => {}
        };
        match *self {
            Pipe::Horizontal | Pipe::Start | Pipe::NorthEast | Pipe::SouthEast => {
                //println!("horizontal lower check");
                if pos.1 < field[0].len() - 1 {
                    let right_pipe: Pipe = field[pos.0][pos.1 + 1];
                    if connects_horizontal(*self, right_pipe) {
                        //println!("add upper neighbor");
                        vec.push((pos.0, pos.1 + 1));
                    } else {
                        //println!("right pipe incompatible with {self:#?}");
                    };
                } else {
                    println!("skip right because {pos:#?}");
                };
            }
            _ => {}
        };
        vec
    }
}
pub fn connects_vertical(top_pipe: Pipe, bottom_pipe: Pipe) -> bool {
    let mut upper: HashSet<Pipe> = HashSet::new();
    let mut lower: HashSet<Pipe> = HashSet::new();

    upper.insert(Pipe::Start);
    upper.insert(Pipe::Vertical);
    upper.insert(Pipe::SouthEast);
    upper.insert(Pipe::SouthWest);

    lower.insert(Pipe::Start);
    lower.insert(Pipe::Vertical);
    lower.insert(Pipe::NorthEast);
    lower.insert(Pipe::NorthWest);

    upper.contains(&top_pipe) && lower.contains(&bottom_pipe)
}

pub fn connects_horizontal(left_pipe: Pipe, right_pipe: Pipe) -> bool {
    let mut lhs: HashSet<Pipe> = HashSet::new();
    let mut rhs: HashSet<Pipe> = HashSet::new();

    lhs.insert(Pipe::Start);
    lhs.insert(Pipe::Horizontal);
    lhs.insert(Pipe::NorthEast);
    lhs.insert(Pipe::SouthEast);

    rhs.insert(Pipe::Start);
    rhs.insert(Pipe::Horizontal);
    rhs.insert(Pipe::NorthWest);
    rhs.insert(Pipe::SouthWest);

    lhs.contains(&left_pipe) && rhs.contains(&right_pipe)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Pipe>> {
    let (input, sequence) = many1(one_of("S|-LJ7F."))(input)?;
    let pipes: Vec<Pipe> = sequence.iter().map(|s| Pipe::new(*s)).collect::<Vec<Pipe>>();
    Ok((input, pipes))
}

fn parse_field(input: &str) -> IResult<&str, Vec<Vec<Pipe>>> {
    let (input, field) = separated_list1(newline, parse_row)(input)?;

    Ok((input, field))
}

pub fn find_start(field: &Vec<Vec<Pipe>>) -> Option<(usize, usize)> {
    for (i, r) in field.iter().enumerate() {
        for (j, p) in r.iter().enumerate() {
            if *p == Pipe::Start {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn exterior_pipes(field: &Vec<Vec<Pipe>>, pipes: &HashSet<(usize, usize)>) -> usize {
    let start: (usize, usize) = (0, 0);
    let mut stack: Vec<(usize, usize)> = vec![start];
    let mut nexterior_pipes: usize = 0usize;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..field[0].len() {
        let pos: (usize, usize) = (0, i);
        if !pipes.contains(&pos) && !visited.contains(&pos) {
            stack.push(pos);
            visited.insert(pos);
        }
        let pos: (usize, usize) = (field.len() - 1, i);
        if !pipes.contains(&pos) && !visited.contains(&pos) {
            stack.push(pos);
            visited.insert(pos);
        }
    }
    for i in 1..field.len() - 1 {
        let pos: (usize, usize) = (i, 0);
        if !pipes.contains(&pos) && !visited.contains(&pos) {
            stack.push(pos);
            visited.insert(pos);
        }
        let pos: (usize, usize) = (i, field[0].len() - 1);
        if !pipes.contains(&pos) && !visited.contains(&pos) {
            stack.push(pos);
            visited.insert(pos);
        }
    }

    while !stack.is_empty() {
        let Some(pos) = stack.pop() else {
            println!("something funky happened");
            break;
        };
        nexterior_pipes += 1;
        if !pipes.contains(&pos) {
            if pos.0 > 0 && !visited.contains(&(pos.0 - 1, pos.1)) {
                let pos: (usize, usize) = (pos.0 - 1, pos.1);
                stack.push(pos);
                visited.insert(pos);
            }
            if pos.0 < field.len() - 1 && !visited.contains(&(pos.0 + 1, pos.1)) {
                let pos: (usize, usize) = (pos.0 + 1, pos.1);
                stack.push(pos);
                visited.insert(pos);
            }
            if pos.1 > 0 && !visited.contains(&(pos.0, pos.1 - 1)) {
                let pos: (usize, usize) = (pos.0, pos.1 - 1);
                stack.push(pos);
                visited.insert(pos);
            }
            if pos.1 < field[0].len() && !visited.contains(&(pos.0, pos.1 + 1)) {
                let pos: (usize, usize) = (pos.0, pos.1 + 1);
                stack.push(pos);
                visited.insert(pos);
            }
        }
    }
    visited.len()
}

pub fn find_loop(field: &Vec<Vec<Pipe>>) -> (usize, HashSet<(usize, usize)>) {
    let mut queue: VecDeque<(usize, Pipe, (usize, usize))> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let Some(s) = find_start(&field) else {
        panic!("no start found");
    };
    let mut max_steps: usize = 0;

    queue.push_back((0, Pipe::Start, s));
    while !queue.is_empty() {
        let Some((step, pipe, pos)) = queue.pop_front() else {
            panic!("unexpected pop");
        };
        visited.insert(pos);
        if step > max_steps {
            max_steps = step;
        };
        println!("step: {step} popped: {pipe:?} at {pos:?}");
        for neighbor in pipe.neighbors(&field, pos) {
            if !visited.contains(&neighbor) {
                let (x, y): (usize, usize) = neighbor;
                let new_pipe: Pipe = field[x][y];
                println!("step: {step}: push {new_pipe:#?} at {neighbor:?}");
                queue.push_back((step + 1, new_pipe, neighbor));
            }
        }
    }

    (max_steps, visited)
}

pub fn count_interior_positions(field: &Vec<Vec<Pipe>>, pipe: HashSet<(usize, usize)>) -> usize {
    let mut interior_positions: Vec<(usize, usize)> = Vec::new();
    let mut ninterior: usize = 0;
    for x in 0..field.len() {
        for y in 0..field[0].len() {
            if pipe.contains(&(x, y)) {
                continue;
            }
            let (mut newx, mut newy) = (x, y);
            let mut ncrossings: usize = 0;

            while newx < field.len() && newy >= 0 {
                let pos: (usize, usize) = (newx, newy);
                let p: Pipe = field[newx][newy];
                let is_corner: bool = p.is_corner();
                if pipe.contains(&pos) && !is_corner {
                    ncrossings += 1;
                }
                if newx == 0 || newy == 0 {
                    break;
                }
                newx = newx - 1;
                newy = newy - 1;
            }
            if ncrossings.rem_euclid(2) == 1 {
                ninterior += 1;
            }
        }
    }

    ninterior
}

impl CommandImpl for Day10 {
    fn main(&self) -> Result<(), DynError> {
        let string = read_to_string(&self.input).unwrap();
        let (_, field) = parse_field(&string).unwrap();
        let Some(start) = find_start(&field) else {
            println!("unable to find start");
            return Ok(());
        };
        println!("field: {field:?}");
        let (max_steps, pipe_positions) = find_loop(&field);
        println!("pipes are at {pipe_positions:?}");
        println!("there are {:?} pipes in the loop", pipe_positions.len());
        let ninterior = count_interior_positions(&field, pipe_positions);
        //println!("sequences {sequences:#?}");
        //let solution: i32 = solve_parta(sequences);
        //println!("solution a: {solution:#?}");
        //println!("start: {start:#?}");
        //println!("max steps: {max_steps}");
        println!("n interior pipes: {ninterior}");
        //let field_dimension = field.len() * field[0].len();
        //println!("field dimension: {field_dimension}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_overlaps() {
        //(&self, pos: (usize, usize), dim: (usize, usize)) -> Vec<(usize, usize)> {
        let pipe: Pipe = Pipe::Start;
        let pos: (usize, usize) = (0, 0);
        let field: Vec<Vec<Pipe>> = vec![
            vec![Pipe::Start, Pipe::Horizontal, Pipe::SouthWest],
            vec![Pipe::Vertical, Pipe::None, Pipe::Vertical],
            vec![Pipe::NorthEast, Pipe::Horizontal, Pipe::NorthWest],
        ];
        let neighbors: Vec<(usize, usize)> = pipe.neighbors(&field, pos);
        assert_eq!(2usize, neighbors.len());
    }
}
