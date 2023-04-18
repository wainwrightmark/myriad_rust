use std::collections::{BinaryHeap, HashSet};

use geometrid::prelude::Tile;
use itertools::Itertools;
use rand::prelude::{SliceRandom, StdRng};

use crate::prelude::*;

#[derive(Clone, Eq, PartialEq)]
struct SolvedBoard<const C: u8, const R: u8, const SIZE: usize> {
    pub board: Board<C, R, SIZE>,
    pub solutions: usize,
}

impl<const C: u8, const R: u8, const SIZE: usize> Ord for SolvedBoard<C, R, SIZE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.solutions
            .cmp(&other.solutions)
            .then_with(|| self.board.to_string().cmp(&other.board.to_string()))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<const C: u8, const R: u8, const SIZE: usize> PartialOrd for SolvedBoard<C, R, SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BoardCreateSettings {
    pub branching_factor: usize,
}

impl BoardCreateSettings {
    pub fn create_boards<const L: u8, const SIZE: usize, GM: GameMode>(
        self,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> impl Iterator<Item = Board<L, L, SIZE>> {
        CreatorIterator::<L, L, SIZE, GM>::new(self, solve_settings, rng)
    }
}

struct CreatorIterator<const C: u8, const R: u8, const SIZE: usize, GM: GameMode> {
    create_settings: BoardCreateSettings,
    solve_settings: SolveSettings,
    desired_solutions: usize,
    rng: StdRng,
    letter_positions: Vec<(usize, Rune)>,

    created_boards: HashSet<String>,
    heap: BinaryHeap<SolvedBoard<C, R, SIZE>>,
    _game_mode: GM,
}

impl<const C: u8, const R: u8, const SIZE: usize, GM: GameMode> CreatorIterator<C, R, SIZE, GM> {
    pub fn new(
        create_settings: BoardCreateSettings,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> Self {
        let board1 = Board::try_create(&str::repeat("_", SIZE)).unwrap();

        let mut heap = BinaryHeap::<SolvedBoard<C, R, SIZE>>::new();
        heap.push(SolvedBoard {
            board: board1,
            solutions: 0,
        });

        let letter_positions = (0..SIZE)
            .cartesian_product(GM::default().legal_letters().iter().cloned())
            .collect_vec();

        Self {
            create_settings,
            desired_solutions: solve_settings.total_solutions(),
            solve_settings,
            rng,
            created_boards: Default::default(),
            heap,
            letter_positions,
            _game_mode: GM::default(),
        }
    }
}

impl<const L: u8, const SIZE: usize, GM: GameMode> Iterator for CreatorIterator<L, L, SIZE, GM> {
    type Item = Board<L, L, SIZE>;

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
                        *letter,
                        *index,
                    )
                });

            for sol in solutions.take(bf) {
                self.heap.push(sol);
            }
        }

        None
    }
}

fn mutate_board<const L: u8, const SIZE: usize>(
    board: &SolvedBoard<L, L, SIZE>,
    solve_settings: SolveSettings,
    created_boards: &mut HashSet<String>,
    letter: Rune,
    index: usize,
) -> Option<SolvedBoard<L, L, SIZE>> {
    let current_letter = board.board[Tile::try_from_usize(index).unwrap()];
    if current_letter == letter {
        return None;
    };

    let mut new_board = board.board.clone();
    new_board[Tile::try_from_usize(index).unwrap()] = letter;

    let unique_string = new_board.canonical_string();
    if created_boards.insert(unique_string) {
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
