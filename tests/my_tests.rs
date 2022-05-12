#[cfg(test)]
use std::cell::RefCell;

use itertools::Itertools;
use myriad::core::prelude::*;

#[test]
fn from_string_test() {
    test_board("98_-7+524", 100);
}

macro_rules! board_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                test_board($value, 100);
            }
        )*
        }
}

board_tests!(
 t1: "98_-7+524",
 t2: "7-6574+2/",
 t3: "-+718325+",
 t4: "7+58-2675",
 t5: "34+*2651+",
 t6: "813+*-372",
 t7: "-+718325+",

);

fn test_board(letters: &str, expected_count: usize) {
    let board = Board::try_create(letters).expect("board should be created");

    let settings = SolveSettings { min: 1, max: 100 };

    let solutions = settings
        .solve(board.clone())
        .collect::<Vec<FoundWord>>();

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
    let solve_settings = SolveSettings { min: 1, max: 100 };
    let one_thousand_solve_settings = SolveSettings { min: 1, max: 1000 };
    let ten_thousand_solve_settings = SolveSettings { min: 1, max: 10000 };

    let settings = BoardCreateSettings {
        branches_to_take: 3,
        desired_solutions: 100,
        number_to_return: 10,
    };
    let rng = rand::SeedableRng::seed_from_u64(100);
    let rng_cell = RefCell::new(rng);

    let boards = &create_boards(solve_settings, 9, &settings, &rng_cell);

    for board in boards {        
        let one_thousand_solutions = one_thousand_solve_settings.solve(board.clone()).count();
        let ten_thousand_solutions = ten_thousand_solve_settings.solve(board.clone()).count();

        eprintln!("{} ({}, {})", board.to_single_string(), one_thousand_solutions, ten_thousand_solutions);
    }

    assert!(boards.len() >= settings.number_to_return);
}
