use std::{num::ParseIntError, fmt::Display, collections::{HashMap, HashSet}};

use thiserror::Error;

use crate::columns::ColumnDisplay;

#[derive(Debug)]
pub struct BingoNumbers(Vec<usize>);

const BOARD_SIZE: usize = 5;
type Board<T> = [[T; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Default)]
pub struct BingoBoard {
    numbers: Board<usize>,
    marks: Board<bool>,
}

// #[derive(Debug)]
// pub enum Bingo {
//     Row(usize),
//     Col(usize),
// }

impl BingoBoard {
    /// Mark a number on board
    fn mark(&mut self, num: usize) {
        let pos = self.numbers.iter()
            .flatten()
            .position(|n| *n == num);
        if let Some(pos) = pos {
            let mark = self.marks.iter_mut()
                .flatten()
                .nth(pos)
                .expect("Lenghts of `numbers` and `marks` are the same");
            *mark = true;
        }
    }

    fn is_bingo(&self) -> bool {
        for row in 0..BOARD_SIZE {
            if self.marks[row].iter().all(|marked| *marked) {
                return true;
            }
        }
        for col in 0..BOARD_SIZE {
            if self.marks.iter().map(|row| row[col]).all(|marked| marked) {
                return true;
            }
        }
        false
    }

    fn score(&self, called_num: usize) -> usize {
        let unmarked = self.numbers.iter().flatten()
            .zip(self.marks.iter().flatten())
            .filter_map(|(num, marked)| if !marked {
                Some(*num)
            } else {
                None
            });
        let sum: usize = unmarked.sum();
        sum * called_num
    }

    // fn is_bingo(&self) -> Option<Bingo> {
    //     for row in 0..BOARD_SIZE {
    //         if self.marks[row].iter().all(|marked| *marked) {
    //             return Some(Bingo::Row(row));
    //         }
    //     }
    //     for col in 0..BOARD_SIZE {
    //         if self.marks.iter().map(|row| row[col]).all(|marked| marked) {
    //             return Some(Bingo::Col(col));
    //         }
    //     }
    //     None
    // }
}

fn play<'a>(nums: &'a BingoNumbers, boards: &'a mut [BingoBoard], verbose: bool, until_n_winners: usize) -> Option<usize> {
    let mut winners_set = HashSet::new();

    for (i, num) in nums.0.iter().enumerate() {
        if verbose {
            println!("Number {} (iter #{}):\n{}",
                num, i, boards.column_display("  ".to_string(), "     ".to_string()));
        }
        for board in boards.iter_mut() {
            board.mark(*num);
        }

        let winners = boards.iter()
            .enumerate()
            .filter(|(_i, b)| b.is_bingo());

        let mut current_winner = None;
        for (i, _b) in winners {
            let is_new = winners_set.insert(i);
            if is_new {
                current_winner = Some(i);
            }
        }

        if let Some(i) = current_winner {
            if winners_set.len() >= until_n_winners {
                assert_eq!(winners_set.len(), until_n_winners, "Multiple boards won at the same time: winners={} vs cond={}",
                    winners_set.len(), until_n_winners);

                let winner = &boards[i];
                let score = winner.score(*num);
                if verbose {
                    println!("Found the board {}!", i);
                    println!("Final state:\n{}",
                        boards.column_display("  ".to_string(), "     ".to_string()));
                }
                return Some(score);
            }
        }

    }
    None
}

pub fn play_to_win<'a>(nums: &'a BingoNumbers, boards: &'a mut [BingoBoard], verbose: bool) -> Option<usize> {
    play(nums, boards, verbose, 1)
}

pub fn play_to_loose<'a>(nums: &'a BingoNumbers, boards: &'a mut [BingoBoard], verbose: bool) -> Option<usize> {
    play(nums, boards, verbose, boards.len())
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.numbers.iter().flatten().max().unwrap();
        let width = max.to_string().len();
        for row in 0..BOARD_SIZE {
            let (nums, marks)  = self.numbers.iter()
                .zip(self.marks.iter())
                .nth(row)
                .unwrap();
            let cols: Vec<_> = nums.iter()
                .zip(marks.iter())
                .map(|(num, is_marked)| {
                    let (start, end) = if *is_marked {
                        ("\x1b[1m", "\x1b[0m")
                    } else {
                        ("", "")
                    };
                    format!("{}{:width$}{}", start, num, end, width = width)
                }).collect();
            if row != 0 {
                writeln!(f, "")?;
            }
            write!(f, "{}", cols.join(" "))?;
        }
        Ok(())
    }
}


#[derive(Error, Debug)]
pub enum InputError {
    #[error("Not enough input lines")]
    MissingInput,
    #[error("No bingo numbers order (maybe missing ',' ?)")]
    NoInputNumbers,
    // #[error("Number of lines with board values not divisible by {}: {0}", BOARD_SIZE)]
    #[error("Wrong number of rows for last board lines: {0}")]
    WrongNumberOfRows(usize),
    #[error("Number of lines with board values not divisible by 5: {0}")]
    WrongNumberOfColumns(usize),
    #[error("Invalid number")]
    ParseNumError(#[from] ParseIntError),
}

/// Parse a list of non-empty lines into bingo task inputs
pub fn parse_lines<S: AsRef<str>>(lines: &[S]) -> Result<(BingoNumbers, Vec<BingoBoard>), InputError> {
    if lines.len() < 1 {
        return Err(InputError::MissingInput);
    }

    let nums = BingoNumbers(lines[0].as_ref()
        .split(',')
        .map(|num| num.parse()
            .map_err(|e| InputError::from(e)))
        .collect::<Result<Vec<_>, InputError>>()?
    );
    if nums.0.len() == 0 {
        return Err(InputError::NoInputNumbers);
    }

    let board_lines = lines[1..].chunks_exact(BOARD_SIZE);
    if !board_lines.remainder().is_empty() {
        return Err(InputError::WrongNumberOfRows(board_lines.remainder().len()));
    }

    let mut boards = Vec::with_capacity(board_lines.len());
    for lines in board_lines {
        assert_eq!(lines.len(), BOARD_SIZE);

        let mut board = BingoBoard::default();
        for (row, line) in lines.iter().enumerate() {
            let tokens: Vec<_> = line.as_ref().split_whitespace().collect();
            if tokens.len() != BOARD_SIZE {
                return Err(InputError::WrongNumberOfColumns(tokens.len()));
            }
            for (col, token) in tokens.iter().enumerate() {
                let num = token.parse()?;
                board.numbers[row][col] = num;
            }
        }

        boards.push(board);
    }

    Ok((nums, boards))
}
