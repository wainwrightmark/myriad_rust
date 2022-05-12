use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;
use num::ToPrimitive;
use rand::prelude::{IteratorRandom, SliceRandom, StdRng};

use crate::core::prelude::*;

use super::{letter, coordinate};

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
    pub branching_factor: usize,
}

impl BoardCreateSettings {
    pub fn create_boards(
        self,
        board_size: usize,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> impl Iterator<Item = Board> {
        CreatorIterator::new(self, board_size, solve_settings, rng)
    }
}

/*
struct CreatorIterator2 {
    create_settings: BoardCreateSettings,
    solve_settings: SolveSettings,
    desired_solutions: usize,
    rng: StdRng,
    created_boards: HashSet<String>,
    heap: BinaryHeap<SolvedBoard>,
    max_coordinate: Coordinate,
    board_size: usize,
}


impl CreatorIterator2 {
    pub fn new(
        create_settings: BoardCreateSettings,
        board_size: usize,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> Self {
        let heap = BinaryHeap::<SolvedBoard>::new();
        let max_coordinate =
            Coordinate::get_max_coordinate_for_square_grid(board_size.to_u8().unwrap());

        Self {
            create_settings,
            desired_solutions: solve_settings.total_solutions(),
            solve_settings,
            rng,
            created_boards: Default::default(),
            heap,
            max_coordinate,
            board_size,
        }
    }

}

impl Iterator for CreatorIterator2 {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(sb) = self.heap.pop() {
                //Check if this is a good board
                if sb.solutions >= self.desired_solutions {
                    return Some(sb.board);
                }

                //TODO don't calculate solutions twice!!!
                let coordinate_counts = self.solve_settings
                .solve(sb.board.clone()).flat_map(|m| m.path)
                .chain(self.max_coordinate.get_positions_up_to()) //add one of each coordinate in case some coordinates never appear
                .counts();

                let worst_coordinate = coordinate_counts.into_iter().min_by_key(|f|f.1).unwrap().0;
                let index: usize = ((worst_coordinate.row * self.max_coordinate.column + 1) + worst_coordinate.column) as usize;

                for new_letter in Letter::legal_letters().choose_multiple(&mut self.rng, self.create_settings.branching_factor)  {
                    mutate_board(&sb, self. solve_settings,&mut self.created_boards, new_letter, index);
                }               
            
                
            } else {
                //Create random boards and start again
                for _ in 0..self.create_settings.branching_factor {
                    let letters = (0..self.board_size)
                        .map(|_| {
                            Letter::legal_letters()
                                .into_iter()
                                .choose(&mut self.rng)
                                .unwrap()
                        })
                        .collect_vec();
        
                    let board1 = Board {
                        columns: self.max_coordinate.column + 1,
                        letters,
                    };
                    self.heap.push(SolvedBoard {
                        board: board1,
                        solutions: 0,
                    });
                }
            }
        }
    }
}
 */

struct CreatorIterator {
    create_settings: BoardCreateSettings,
    solve_settings: SolveSettings,
    desired_solutions: usize,
    rng: StdRng,
    letter_positions: Vec<(usize, Letter)>,

    created_boards: HashSet<String>,
    heap: BinaryHeap<SolvedBoard>,
}

impl CreatorIterator {
    pub fn new(
        create_settings: BoardCreateSettings,
        board_size: usize,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> Self {
        let board1 = Board::try_create(&str::repeat("_", board_size)).unwrap();

        let mut heap = BinaryHeap::<SolvedBoard>::new();
        heap.push(SolvedBoard {
            board: board1,
            solutions: 0,
        });

        let letter_positions = (0..board_size)
            .cartesian_product(Letter::legal_letters().collect_vec())
            .collect_vec();

        Self {
            create_settings,
            desired_solutions: solve_settings.total_solutions(),
            solve_settings,
            rng,
            created_boards: Default::default(),
            heap,
            letter_positions,
        }
    }
}

impl Iterator for CreatorIterator {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sb) = self.heap.pop() {
            //Check if this is a good board
            if sb.solutions >= self.desired_solutions {
                return Some(sb.board);
            }

            //It is not a good board - mutate it
            let board = &sb;
            let bf = self.create_settings.branching_factor;

            let solutions = self
                .letter_positions
                .choose_multiple(&mut self.rng, bf * 2)
                .filter_map(|(index, letter)| {
                    mutate_board(
                        board,
                        self.solve_settings,
                        &mut self.created_boards,
                        letter.clone(),
                        index.clone(),
                    )
                });

            for sol in solutions.take(bf) {
                self.heap.push(sol);
            }
        }

        None
    }
}

fn mutate_board(
    board: &SolvedBoard,
    solve_settings: SolveSettings,
    created_boards: &mut HashSet<String>,
    letter: Letter,
    index: usize,
) -> Option<SolvedBoard> {
    let current_letter = board.board.get_letter_at_index(index);
    if current_letter == letter {
        return None;
    };

    let new_board = board.board.with_set_letter(letter, index);

    if created_boards.insert(new_board.get_unique_string()) {
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
