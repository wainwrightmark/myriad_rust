#[cfg(test)]
use itertools::Itertools;
use myriad::core::prelude::*;

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

    let solutions = settings.solve(board.clone()).collect::<Vec<FoundWord>>();

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
        branching_factor: 1,
    };
    let rng = rand::SeedableRng::seed_from_u64(100);

    let boards = settings
        .create_boards(9, solve_settings, rng)
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
