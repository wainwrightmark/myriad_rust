use clap::Parser;
use itertools::Itertools;
use myriad::core::prelude::*;
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

    // let board_create_settings = BoardCreateSettings {
    //     branching_factor: args.branching_factor,
    // };
    // let seed: u64 = rand::random();

    //let rng = rand::SeedableRng::seed_from_u64(seed);

    // let mut boards = board_create_settings
    //     .create_boards::<3, 3>(solve_settings, rng)
    //     .take(args.take)
    //     .map(|x| x.to_single_string());

    use Rune::*;
    let iterator = BoardIterator {
        current: Board {
            runes: [[One, Two, Three], [Four, Five, Six], [Seven, Eight, Nine]],
        }, //todo start earlier
    };

    let mut boards = iterator
        .filter(filter_good)
        //TODO use parallel
        .map(|b| {
            let s = b.to_single_string();
            let solutions = solve_settings.solve(b.clone()).count();

            format!("{}: {}", s, solutions)
        })
        .take(args.take);

    let text = boards.join("\r\n");

    fs::write("boards.txt", text).expect("Unable to write file");
}

pub fn filter_good(board: &Board<3, 3>) -> bool {
    use Rune::*;
    if board.runes[0][0] > board.runes[0][2] {
        return false;
    };
    if board.runes[0][2] > board.runes[2][0] {
        return false;
    };
    if board.runes[2][0] > board.runes[2][2] {
        return false;
    };

    //TODO more rotation restrictions

    let mut non_zero_digits = 0;
    let mut sum_operators = 0;
    //let mut product_operators = 0;

    for row in 0..3 {
        for column in 0..3 {
            let rune = board.runes[column][row];

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
                }
                //Times => {product_operators += 1;},
                Minus => {
                    sum_operators += 1;
                }
                //Divide => {product_operators += 1;},
                Blank => {}
                _ => {}
            }
        }
    }

    if non_zero_digits < 4 {
        return false;
    }
    if sum_operators < 1 {
        return false;
    }

    return true;
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct BoardIterator {
    pub current: Board<3, 3>,
}

impl Iterator for BoardIterator {
    type Item = Board<3, 3>;

    fn next(&mut self) -> Option<Self::Item> {
        for column in (0..3).rev() {
            for row in (0..3).rev() {
                if let Some(next_value) = Self::get_next(self.current.runes[column][row]) {
                    self.current.runes[column][row] = next_value;
                    return Some(self.current.clone());
                } else {
                    self.current.runes[column][row] = Self::first_rune()
                }
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
        };
        Some(n)
    }
}
