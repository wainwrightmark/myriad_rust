#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(test)]
use itertools::Itertools;
use myriad::core::prelude::*;
use ntest::test_case;

#[test_case("98_-7+524")]
//#[test_case("7-6574+2/")]
#[test_case("-+718325+")]
#[test_case("7+58-2675")]
//#[test_case("34+*2651+")]
#[test_case("813+*-372")]
#[test_case("6+98161-3")]
#[test_case("-9+1236+5")]
#[test_case("+-389-425")]
//#[test_case("/3+421+58")]
//#[test_case("6973-*718")]
#[test_case("6258+-73-")]
#[test_case("455+695-3")]
#[test_case("1+536-249")]
#[test_case("4+726-*49")]
//#[test_case("*6++47321")]
#[test_case("129657-+3")]
#[test_case("/97-1+463")]
fn test_board_100(letters: &str) {
    test_board(letters, 100)
}

fn test_board(letters: &str, expected_count: usize) {
    let board = Board::<3, 3>::try_create(letters).expect("board should be created");

    let settings = SolveSettings { min: 1, max: 100 };

    let solutions = settings.solve(board.clone()).collect::<Vec<FoundWord<3,3>>>();

    for r in solutions
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.result, &b.result))
    {
        let coordinates = r.path.clone();
        let word_text = board.get_word_text(&coordinates);
        eprintln!("{} = {}", r.result, word_text);
    }

    assert_eq!(expected_count, solutions.len());
}

#[test]
fn test_create_boards() {
    let number_to_return = 1;
    let solve_settings = SolveSettings { min: 1, max: 100 };
    let one_thousand_solve_settings = SolveSettings { min: 1, max: 1000 };
    let ten_thousand_solve_settings = SolveSettings { min: 1, max: 10000 };

    let settings = BoardCreateSettings {
        branching_factor: 2,
    };
    let rng = rand::SeedableRng::seed_from_u64(100);

    let boards = settings
        .create_boards::<3, 3>(solve_settings, rng)
        .take(number_to_return)
        .collect_vec();

    assert!(boards.len() >= number_to_return);

    for board in boards {
        let one_thousand_solutions = one_thousand_solve_settings.solve(board.clone()).count();
        let ten_thousand_solutions = ten_thousand_solve_settings.solve(board.clone()).count();

        eprintln!(
            "{} ({}, {})",
            board.to_single_string(),
            one_thousand_solutions,
            ten_thousand_solutions
        );
    }
}

#[test]
pub fn test_type_sizes() {
    let letter = std::mem::size_of::<Rune>();
    let coordinate = std::mem::size_of::<Coordinate<3,3>>();
    let board = std::mem::size_of::<Board<3, 3>>();

    println!("Size of letter: {letter}");
    println!("Size of coordinate: {coordinate}");
    println!("Size of board: {board}");
}
