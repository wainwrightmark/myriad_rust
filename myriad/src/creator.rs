use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;
use rand::prelude::{SliceRandom, StdRng};

use crate::prelude::*;

#[derive(Clone, Eq, PartialEq)]
struct SolvedBoard<const C: usize, const R: usize>
where
    [(); C * R]:,
{
    pub board: Board<C, R>,
    pub solutions: usize,
}

impl<const C: usize, const R: usize> Ord for SolvedBoard<C, R>
where
    [(); C * R]:,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.solutions
            .cmp(&other.solutions)
            .then_with(|| self.board.to_string().cmp(&other.board.to_string()))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<const C: usize, const R: usize> PartialOrd for SolvedBoard<C, R>
where
    [(); C * R]:,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BoardCreateSettings {
    pub branching_factor: usize,
}

impl BoardCreateSettings {
    pub fn create_boards<const C: usize, const R: usize>(
        self,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> impl Iterator<Item = Board<C, R>>
    where
        [(); C * R]:,
    {
        CreatorIterator::new(self, solve_settings, rng)
    }
}

struct CreatorIterator<const C: usize, const R: usize>
where
    [(); C * R]:,
{
    create_settings: BoardCreateSettings,
    solve_settings: SolveSettings,
    desired_solutions: usize,
    rng: StdRng,
    letter_positions: Vec<(usize, Rune)>,

    created_boards: HashSet<String>,
    heap: BinaryHeap<SolvedBoard<C, R>>,
}

impl<const C: usize, const R: usize> CreatorIterator<C, R>
where
    [(); C * R]:,
{
    pub fn new(
        create_settings: BoardCreateSettings,
        solve_settings: SolveSettings,
        rng: StdRng,
    ) -> Self {
        let board_size = C * R;

        let board1 = Board::try_create(&str::repeat("_", board_size)).unwrap();

        let mut heap = BinaryHeap::<SolvedBoard<C, R>>::new();
        heap.push(SolvedBoard {
            board: board1,
            solutions: 0,
        });

        let letter_positions = (0..board_size)
            .cartesian_product(ClassicGameMode {}.legal_letters().iter().cloned())
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

impl<const C: usize, const R: usize> Iterator for CreatorIterator<C, R>
where
    [(); C * R]:,
{
    type Item = Board<C, R>;

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

fn mutate_board<const C: usize, const R: usize>(
    board: &SolvedBoard<C, R>,
    solve_settings: SolveSettings,
    created_boards: &mut HashSet<String>,
    letter: Rune,
    index: usize,
) -> Option<SolvedBoard<C, R>>
where
    [(); C * R]:,
{
    let current_letter = board.board[Coordinate(index as u8)];
    if current_letter == letter {
        return None;
    };

    let mut new_board = board.board.clone();
    new_board[Coordinate(index as u8)] = letter;

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
