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
    pub branching_factor: usize,
}

impl BoardCreateSettings{
    pub fn create_boards(self, board_size: usize,        
        solve_settings: SolveSettings,
        rng: StdRng) ->impl Iterator<Item = Board>{
            CreatorIterator::new(self, board_size, solve_settings, rng)
        }
}


struct CreatorIterator{
    create_settings: BoardCreateSettings,
    solve_settings: SolveSettings,
    desired_solutions: usize,
    board_size: usize,
    rng: StdRng,

    created_boards: HashSet::<String>,
    heap: BinaryHeap::<SolvedBoard>
}

impl CreatorIterator{
    pub fn new(
        create_settings: BoardCreateSettings,
        board_size: usize,        
        solve_settings: SolveSettings,
        rng: StdRng,
    )-> Self{

        let board1 = Board::try_create(&str::repeat("_", board_size)).unwrap();                        
    
        let mut heap = BinaryHeap::<SolvedBoard>::new();
        heap.push(SolvedBoard {
            board: board1,
            solutions: 0,
        });


        Self { 
            create_settings, 
            desired_solutions: solve_settings.total_solutions(), 
            solve_settings,             
            board_size, 
            rng, 
            created_boards : Default::default(), 
            heap
         }

    }

    fn get_next_boards(&mut self,
        board : &SolvedBoard
    ) -> Vec<SolvedBoard> 
    {        
        let bf = self.create_settings.branching_factor;

        let mut indexes = (0..self.board_size)
            .cartesian_product(Letter::legal_letters().collect_vec())
            .collect_vec();
        indexes.shuffle(&mut self.rng);

        let solutions = indexes.into_iter().filter_map(|(index, letter)| {
            self.mutate_board(board, letter, index)
        });
        solutions.take(bf).collect()
    }

    fn mutate_board(
        &mut self,
        board: &SolvedBoard,
        letter: Letter,
        index: usize
    ) -> Option<SolvedBoard> {
        let current_letter = board.board.get_letter_at_index(index);
        if current_letter == letter {
            return None;
        };

        let new_board = board.board.with_set_letter(letter, index);

        if self.created_boards.insert(new_board.get_unique_string()) {

            let solutions = self.solve_settings.solve(new_board.clone());            
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

impl Iterator for CreatorIterator{
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(sb) = self.heap.pop() {

            //Check if this is a solution
            if sb.solutions >= self.desired_solutions{
                return Some(sb.board);
            }           

            //It is not a solution - mutate it
            let solutions = self.get_next_boards(&sb);    
            self.heap.append(&mut BinaryHeap::from(solutions.clone()));    
    
        }

        None
    }
}
