use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::fs::read_to_string;

use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Day4 {
    #[clap(long, short)]
    input: PathBuf,
}

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline, space0, space1},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
pub struct Board {
    lookup: HashMap<u32, u32>,
    board: u32,
}

impl Board {
    fn new(lookup: Vec<u32>) -> Self {
        let lookup: HashMap<u32, u32> = lookup
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i as u32))
            .collect::<HashMap<u32, u32>>();
        Board { lookup, board: 0u32 }
    }

    fn print_board(&self) {
        for i in 0..25 {
            if ((self.board >> i as u32) & 1u32 == 1u32) {
                print!("x");
            } else {
                print!("o");
            }
            if i % 5 == 4 {
                println!("");
            }
        }
    }

    fn draw(&mut self, number: u32) {
        match self.lookup.get(&number) {
            Some(s) => {
                let shiftn: u32 = 1u32 << s;
                self.board = self.board | shiftn;
                let board = self.board;
            }
            None => return,
        };
        self.lookup.remove(&number);
    }

    fn bingo(&self) -> bool {
        // check for completed row
        for i in 0..5 {
            let board = self.board >> i;
            if board & COLUMN == COLUMN {
                return true;
            }
        }

        // check for completed coumn
        let mut board = self.board;
        for i in 0..5 {
            let board = self.board >> (i * 5);
            if board & ROW == ROW {
                return true;
            }
        }
        return false;
    }

    fn play_bingo(&mut self, draw: &[u32]) -> (u32, u32) {
        let mut winning_number: u32 = 0;
        let mut winning_round: u32 = draw.len() as u32;
        for (i, x) in draw.iter().enumerate() {
            self.draw(*x);
            if self.bingo() {
                winning_number = *x;
                winning_round = i as u32;
                break;
            }
        }
        let sum: u32 = self.lookup.keys().sum();
        (winning_round, sum * winning_number)
    }
}

#[derive(Debug)]
pub struct BingoGame {
    draw: Vec<u32>,
    boards: Vec<Board>,
}

impl BingoGame {
    pub fn new(draw: Vec<u32>, boards: Vec<Board>) -> Self {
        BingoGame { draw, boards }
    }

    pub fn print_board(&self, board_number: usize) {
        println!("{:?}", self.boards[board_number]);
    }

    pub fn peek_board(&self, board_number: usize) -> &Board {
        &self.boards[board_number]
    }

    pub fn print_draw(&self) {
        println!("{:?}", self.draw);
    }

    pub fn play_bingo(&mut self) -> (u32, u32) {
        let first_draw = &self.draw[..];
        let scores: Vec<(u32,u32)> = self.boards
            .iter_mut()
            .map(|b| b.play_bingo(first_draw))
            .collect::<Vec<(u32,u32)>>();
        let winning_board = scores
            .iter()
            .min_by_key(|x| &x.0)
            .unwrap();
        let losing_board = scores
            .iter()
            .max_by_key(|x| &x.0)
            .unwrap();
        let winning_round: u32 = winning_board.0;
        let losing_round: u32 = losing_board.0;
        let winners = scores
            .iter()
            .filter(|x| x.0 == winning_round)
            .max_by_key(|x| &x.1)
            .unwrap();
        let losers = scores
            .iter()
            .filter(|x| x.0 == losing_round)
            .min_by_key(|x| &x.1)
            .unwrap();
        (winners.1, losers.1)
    }
}

fn parse_draw(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, items) = separated_list1(tag(","), nom::character::complete::u32)(input)?;
    Ok((input, items))
}

fn parse_board_row(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = space0(input)?;
    let (input, row) = separated_list1(space1, nom::character::complete::u32)(input)?;
    Ok((input, row))
}

fn parse_board(input: &str) -> IResult<&str, Board> {
    let (input, v) = separated_list1(newline, parse_board_row)(input)?;
    //let (input, v) = many1(parse_board_row)(input)?;
    let mut lookup: Vec<u32> = Vec::new();
    for (i, x) in v.iter().flatten().enumerate() {
        lookup.push(*x);
    }
    let board: Board = Board::new(lookup);
    Ok((input, board))
}

fn parse_boards(input: &str) -> IResult<&str, Vec<Board>> {
    let (input, vecs) =
        separated_list1(parse_newline, parse_board)(input).unwrap();
    Ok((input, vecs))
}

fn parse_bingo_game(input: &str) -> IResult<&str, BingoGame> {
    let (i, draw) = parse_draw(input).unwrap();
    let (i, _) = parse_newline(&i).unwrap();
    let boards = parse_boards(&i).unwrap();
    println!("read a total of {:?} boards", boards.1.len());
    let bingo_game: BingoGame = BingoGame::new(draw, boards.1);
    Ok((input, bingo_game))
}

fn parse_newline(input: &str) -> IResult<&str, Vec<char>> {
    many1(newline)(input)
}

const ROW: u32 = 0b00011111u32;
const COLUMN: u32 = 0b0000100001000010000100001u32;

impl CommandImpl for Day4 {
    fn main(&self) -> Result<(), DynError> {
        let i = read_to_string(&self.input).unwrap();
        let (_, mut bingo_game) = parse_bingo_game(&i).unwrap(); 
        let results = bingo_game.play_bingo();
        println!("winner {:?}, loser {:?}", results.0, results.1);

        //bingo_game.print_draw();
        //bingo_game.peek_board(10).print_board();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_board() {
        let x: &str = "27 67 81 30 95
48 89  7  4  3
82 90 78 85 44
22 16 97 92 11
15 87 47 79 62";
        let (_, board) = parse_board(x).unwrap();
        assert_eq!(board.lookup[&27u32], 0u32);
        assert_eq!(board.lookup[&67u32], 1u32);
    }
    #[test]
    fn test_parse_boards() {
        let x: &str = "27 67 81 30 95
48 89  7  4  3
82 90 78 85 44
22 16 97 92 11
15 87 47 79 62

49 51 35 87 75
 3 70  8 43  5
77 88 73 81 29
42 62 50 37 85
26 86 14 38 65

81  9 84  3 37
33 32  1 54 45
39 83 82 36  2
56 28 76 85 40
96 69 43 24 71";
        let (_, boards) = parse_boards(x).unwrap();
        assert_eq!(boards[0].lookup[&27u32], 0u32);
        assert_eq!(boards[1].lookup[&49u32], 0u32);
    }

    #[test]
    fn test_play_bingo_row() {
        let numbers: Vec<u32> = vec![27, 67, 81, 30, 95];
        let x: &str = "27 67 81 30 95
48 89  7  4  3
82 90 78 85 44
22 16 97 92 11
15 87 47 79 62

49 51 35 87 75
 3 70  8 43  5
77 88 73 81 29
42 62 50 37 85
26 86 14 38 65";
    let (_, mut boards) = parse_boards(x).unwrap();
    let draw = &numbers[..];
    let results = boards[0].play_bingo(draw);
    assert_eq!(results.0, 4u32);
    }

    #[test]
    fn test_play_bingo_column() {
        let numbers: Vec<u32> = vec![75, 5, 29, 85, 65];
        let x: &str = "27 67 81 30 95
48 89  7  4  3
82 90 78 85 44
22 16 97 92 11
15 87 47 79 62

49 51 35 87 75
 3 70  8 43  5
77 88 73 81 29
42 62 50 37 85
26 86 14 38 65";
    let (_, mut boards) = parse_boards(x).unwrap();
    let draw = &numbers[..];
    let results = boards[1].play_bingo(draw);
    assert_eq!(results.0, 4u32);
    }
}
