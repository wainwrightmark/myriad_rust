use itertools::Itertools;
use myriad::core::prelude::*;
use clap::Parser;
use std::{
    fs
};


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

    let board_create_settings = BoardCreateSettings {
        branching_factor: args.branching_factor,
    };
    let seed: u64 = rand::random();

    let rng = rand::SeedableRng::seed_from_u64(seed);

    let mut boards = board_create_settings
        .create_boards::<3, 3>(solve_settings, rng)
        .take(args.take).map(|x|x.to_single_string()) ;

    let text = boards.join("\r\n");
    

    fs::write("boards.txt", text).expect("Unable to write file");
}
