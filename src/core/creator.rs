use std::cell::RefCell;
use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;
use rand::prelude::{SliceRandom, StdRng};

use crate::core::prelude::*;

#[derive(Clone, Eq, PartialEq)]
struct SolvedBoard {
    pub board: Board,
    pub solutions: usize,
}

impl Ord for SolvedBoard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.solutions
            .cmp(&other.solutions)
            .then_with(|| self.board.to_string().cmp(&other.board.to_string()))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for SolvedBoard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BoardCreateSettings {
    pub number_to_return: usize,
    pub branching_factor: usize,
}


// struct CreatorIterator{
//     create_settings: BoardCreateSettings,
//     solve_settings: SolveSettings,
//     size: usize,
//     rng: &RefCell<StdRng>,

//     created_boards: HashSet::<String>,
//     heap: BinaryHeap::<SolvedBoard>
// }

pub fn create_boards(
    solve_settings: SolveSettings,
    size: usize,
    board_create_settings: &BoardCreateSettings,
    rng: &RefCell<StdRng>,
) -> Vec<Board> {
    let board1 = Board::try_create(&str::repeat("_", size)).unwrap();
    let desired_solutions = solve_settings.total_solutions();

    let mut results = Vec::<Board>::new();

    let mut existing_boards = HashSet::<String>::new();
    existing_boards.insert(board1.get_unique_string());

    let mut heap = BinaryHeap::<SolvedBoard>::new();
    heap.push(SolvedBoard {
        board: board1,
        solutions: 0,
    });

    while let Some(sb) = heap.pop() {
        let solutions = get_all_solutions(&sb, solve_settings, rng, &mut existing_boards, board_create_settings);

        heap.append(&mut BinaryHeap::from(solutions.clone()));

        results.append(
            &mut solutions
                .into_iter()
                .filter(|b| b.solutions >= desired_solutions)
                .map(|b| b.board)
                .collect_vec(),
        );

        if results.len() >= board_create_settings.number_to_return {
            return results;
        }
    }

    return results;

    fn get_all_solutions(
        board: &SolvedBoard,
        solve_settings: SolveSettings,
        rng: &RefCell<StdRng>,
        existing_boards: &mut HashSet<String>,
        settings: &BoardCreateSettings,
    ) -> Vec<SolvedBoard> //TODO replace this with an iterator
    {
        let mut r = rng.borrow_mut().to_owned();

        let mut indexes = (0..board.board.letters.len())
            .cartesian_product(Letter::legal_letters().collect_vec())
            .collect_vec();
        indexes.shuffle(&mut r);

        let solutions = indexes.into_iter().filter_map(|(index, letter)| {
            get_better_solutions(board, solve_settings, letter, index, existing_boards)
        });
        solutions.take(settings.branching_factor).collect()
    }

    fn get_better_solutions(
        board: &SolvedBoard,
        solve_settings: SolveSettings,
        letter: Letter,
        index: usize,
        existing_boards: &mut HashSet<String>,
    ) -> Option<SolvedBoard> {
        let current_letter = board.board.get_letter_at_index(index);
        if current_letter == letter {
            return None;
        };

        let new_board = board.board.with_set_letter(letter, index);

        if existing_boards.insert(new_board.get_unique_string()) {

            let solutions = solve_settings.solve(new_board.clone());            
            let solution_count = solutions.count();

            if solution_count >= board.solutions {
                return Some(SolvedBoard {
                    board: new_board,
                    solutions: solution_count,
                });
            }
        }

        None
    }
}
