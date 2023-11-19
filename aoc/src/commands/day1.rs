use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Parser, Debug)]
pub struct Day1 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        let file = File::open(&self.input).expect("File error");
        let reader = BufReader::new(file);

        let numbers: Vec<u64> =
            reader.lines().map(|line| line.unwrap().parse::<u64>().unwrap()).collect();
        let n_increases: u64 = numbers.windows(2).map(|w| if w[1] > w[0] { 1 } else { 0 }).sum();
        let partb: Vec<u64> = numbers.windows(3).map(|w| w[0] + w[1] + w[2]).collect::<Vec<u64>>();
        let n_partb_increases: u64 =
            partb.windows(2).map(|w| if w[1] > w[0] { 1 } else { 0 }).sum();
        //.map(|w| if w[1] > w[1] { 0 } else { 1 })
        //.sum();
        println!("part a: {:?}", n_increases);
        println!("part b: {:?}", n_partb_increases);

        Ok(())
    }
}
