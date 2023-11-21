use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::io::{prelude::*, BufReader};

use std::fs::File;

#[derive(Parser, Debug)]
pub struct Day3 {
    #[clap(long, short)]
    input: PathBuf,
}

fn init_set_bits(binary_string: &str) -> Vec<usize> {
    vec![0usize; binary_string.trim().len()]
}

fn init_binary_rank(len: usize) -> Vec<(usize, usize)> {
    vec![(0usize, 0usize); len]
}

fn gamma(set_bits: &Vec<usize>, nbits: usize) -> usize {
    let mut gamma: u16 = 0u16;
    set_bits
        .iter()
        .rev()
        .enumerate()
        .filter(|&(i, x)| 2 * x > nbits)
        .fold(0, |gamma, (i, _)| gamma | 2_usize.pow(i as u32))
}

fn epsilon(set_bits: &Vec<usize>, nbits: usize) -> usize {
    let mut gamma: u16 = 0u16;
    set_bits
        .iter()
        .rev()
        .enumerate()
        .filter(|&(i, x)| 2 * x <= nbits)
        .fold(0, |gamma, (i, _)| gamma | 2_usize.pow(i as u32))
}

fn get_most_common_bit_setting(bits: &Vec<usize>, nsettings: usize) -> Vec<bool> {
    let nsettings: usize = bits.len();
    let common_setting: Vec<bool> = bits.iter().map(|x| 2 * x >= nsettings).collect::<Vec<bool>>();
    common_setting
}

fn get_shifted_mask(shift: usize) -> usize {
    1usize << shift
}

fn get_most_common_bit(bits: &Vec<usize>, mask: usize) -> usize {
    let tally: usize = bits.iter().map(|b| if b & mask == mask { 1 } else { 0 }).sum();
    if 2 * tally >= bits.len() {
        mask
    } else {
        0usize
    }
}

fn get_least_common_bit(bits: &Vec<usize>, mask: usize) -> usize {
    let tally: usize = bits.iter().map(|b| if b & mask == mask { 1 } else { 0 }).sum();
    if 2 * tally < bits.len() {
        mask
    } else {
        0usize
    }
}

impl CommandImpl for Day3 {
    fn main(&self) -> Result<(), DynError> {
        let file: File = std::fs::File::open(&self.input).expect("unable to open file for reading");
        let mut buf = BufReader::new(file);
        let mut first_line = String::new();
        let _ = buf.read_line(&mut first_line)?;
        let mut set_bits = init_set_bits(&first_line);

        let mut nlines: usize = 0;
        let shiftn: usize = set_bits.len();
        let mut binaries: Vec<usize> = vec![];

        let file: File = std::fs::File::open(&self.input).expect("unable to open file for reading");
        let mut buf = BufReader::new(file);
        for line in buf.lines() {
            let mut intval = usize::from_str_radix(&line?, 2)?;
            for i in 0..set_bits.len() {
                let curr_shift: usize = 1 << (shiftn - i - 1);
                if &intval & curr_shift > 0 {
                    set_bits[i] += 1;
                }
            }
            binaries.push(intval);
            nlines += 1;
        }
        println!("read {:?} binaries", binaries.len());
        let mut common_bits = get_most_common_bit_setting(&set_bits, binaries.len());

        let gam = gamma(&set_bits, nlines);
        let eps = epsilon(&set_bits, nlines);

        let mut o2_rating: usize = 0;
        let mut co2_rating: usize = 0;

        let mut o2_candidates: Vec<usize> = binaries.clone();
        let mut co2_candidates: Vec<usize> = binaries.clone();

        let mut nbits: usize = shiftn - 1;
        loop {
            let mask: usize = get_shifted_mask(nbits);
            let common_mask: usize = get_most_common_bit(&o2_candidates, mask);
            o2_candidates = o2_candidates
                .into_iter()
                .filter(|x| if common_mask > 0 { x & mask == mask } else { x & mask == 0 })
                .collect();
            if o2_candidates.len() == 1 {
                o2_rating = o2_candidates[0];
                break;
            }
            if nbits == 0 {
                println!("unable to calculate O2 generator rating");
                break;
            }
            nbits -= 1;
        }

        let mut nbits: usize = shiftn - 1;
        loop {
            let mask: usize = get_shifted_mask(nbits);
            let common_mask: usize = get_least_common_bit(&co2_candidates, mask);
            co2_candidates = co2_candidates
                .into_iter()
                .filter(|x| if common_mask > 0 { x & mask == mask } else { x & mask == 0 })
                .collect();
            if co2_candidates.len() == 1 {
                co2_rating = co2_candidates[0];
                break;
            }
            if nbits == 0 {
                println!("unable to calculate CO2 generator rating");
                break;
            }
            nbits -= 1;
        }

        println!("------part 1:");
        println!("epsilon: {eps}");
        println!("gamma: {gam}");
        println!("epsilon * gamma: {:?}", eps * gam);
        println!("------part 2:");
        println!("O2 generator rating: {o2_rating}");
        println!("CO2 generator rating: {co2_rating}");
        println!("O2 generator rating * CO2 generator rating: {:?}", o2_rating * co2_rating);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn common_bit() {
        let bits: Vec<usize> = vec![16, 17, 24, 8, 0];
        let mask: usize = get_shifted_mask(4);
        assert_eq!(get_most_common_bit(&bits, mask), mask);
        let mask: usize = get_shifted_mask(3);
        assert_eq!(get_most_common_bit(&bits, mask), 0usize);
    }
}
