#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use clap::Parser;
use itertools::Itertools;
use myriad::prelude::*;
//use rayon::prelude::*;
use std::fs;

/// Generates Myriad boards
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Min length of words to use
    #[clap(short, long, value_parser, default_value_t = 3)]
    branching_factor: usize,
    #[clap(short, long, value_parser, default_value_t = 100)]
    take: usize,
}

pub fn main() {
    let solve_settings = SolveSettings { min: 1, max: 100 };
    let args = Args::parse();

    use Rune::*;
    let iterator = BoardIterator {
        current: Board { runes: [One; 9] }, //todo start earlier
    };

    //let par_iter = iter::split(iterator, 100);

    let mut boards = iterator
        .filter(filter_good)
        //TODO use parallel
        .map(|b| {
            let s = b.to_single_string();
            let solutions = solve_settings.solve(b).count();

            format!("{}: {}", s, solutions)
        })
        .take(args.take);

    let text = boards.join("\r\n");

    fs::write("boards.txt", text).expect("Unable to write file");
}

pub fn filter_good(board: &Board<3, 3>) -> bool {
    use Rune::*;
    if !board.is_canonical_form() {
        return false;
    }

    let mut non_zero_digits = 0;
    let mut sum_operators = 0;
    let mut _product_operators = 0;
    let mut _positive_operators = 0;
    let mut _negative_operators = 0;

    for rune in board.runes {
        match rune {
            //Rune::Zero => {},
            One => {
                non_zero_digits += 1;
            }
            Two => {
                non_zero_digits += 1;
            }
            Three => {
                non_zero_digits += 1;
            }
            Four => {
                non_zero_digits += 1;
            }
            Five => {
                non_zero_digits += 1;
            }
            Six => {
                non_zero_digits += 1;
            }
            Seven => {
                non_zero_digits += 1;
            }
            Eight => {
                non_zero_digits += 1;
            }
            Nine => {
                non_zero_digits += 1;
            }
            Plus => {
                sum_operators += 1;
                _positive_operators += 1;
            }
            Times => {
                _product_operators += 1;
                _positive_operators += 1;
            }
            Minus => {
                sum_operators += 1;
                _negative_operators += 1;
            }
            Divide => {
                _product_operators += 1;
                _negative_operators += 1;
            }
            Blank => {}
            _ => {}
        }
    }

    if non_zero_digits < 4 {
        return false;
    }
    if sum_operators < 1 {
        return false;
    }

    //if negative_operators == 0 && !board.runes

    true
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct BoardIterator {
    pub current: Board<3, 3>,
}

impl Iterator for BoardIterator {
    type Item = Board<3, 3>;

    fn next(&mut self) -> Option<Self::Item> {
        for index in 0..9 {
            if let Some(next_value) = Self::get_next(self.current.runes[index]) {
                self.current.runes[index] = next_value;
                return Some(self.current.clone());
            } else {
                self.current.runes[index] = Self::first_rune()
            }
        }

        None
    }
}

impl BoardIterator {
    pub fn first_rune() -> Rune {
        Rune::Zero
    }
    pub fn get_next(rune: Rune) -> Option<Rune> {
        use Rune::*;
        let n = match rune {
            Zero => One,
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Plus,
            Plus => Times,
            Times => Minus,
            Minus => Divide,
            Divide => Blank,
            Blank => return None,

            _ => return None,
        };
        Some(n)
    }
}
