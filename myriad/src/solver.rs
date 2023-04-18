use crate::parser::ParseFail;
use crate::prelude::*;
use geometrid::prelude::Tile;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    num::NonZeroU8,
};
use tinyvec::ArrayVec;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SolveSettings {
    ///Inclusive minimum
    pub min: i32,
    ///Inclusive maximum
    pub max: i32,
}

impl SolveSettings {
    pub fn allow(&self, num: i32) -> bool {
        self.min <= num && num <= self.max
    }

    ///Get all solutions to this board within the range
    pub fn solve<const C: u8, const R: u8, const SIZE: usize>(
        self,
        board: Board<C, R, SIZE>,
    ) -> impl Iterator<Item = FoundWord<C, R, SIZE>> {
        SolutionIter::new(board, self)
    }

    pub fn total_solutions(&self) -> usize {
        (self.max - self.min + 1) as usize
    }
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self { min: 1, max: 100 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Path<const C: u8, const R: u8, const SIZE: usize> {
    tiles: ArrayVec<[Tile<C, R>; SIZE]>,
    used: TileSet16<C, R, SIZE>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct FoundWord<const C: u8, const R: u8, const SIZE: usize> {
    pub result: i32,
    pub path: ArrayVec<[Tile<C, R>; SIZE]>,
}

impl<const C: u8, const R: u8, const SIZE: usize> std::fmt::Display for FoundWord<C, R, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.result, self.path.iter().join(""))
    }
}

impl FoundWord<3, 3, 9> {
    pub fn get_difficulty(&self) -> Difficulty {
        if self.path.is_empty() {
            panic!("Empty word cannot have difficulty")
        } else if self.path.len() <= 9 {
            Difficulty(NonZeroU8::new(self.path.len() as u8).unwrap())
        } else {
            panic!("Word has wrong path length to have difficulty");
        }
    }
}

struct SolutionIter<const C: u8, const R: u8, const SIZE: usize> {
    results: HashSet<i32>,
    settings: SolveSettings,
    queue: VecDeque<Path<C, R, SIZE>>,
    board: Board<C, R, SIZE>,
}

impl<const C: u8, const R: u8, const SIZE: usize> SolutionIter<C, R, SIZE> {
    pub fn new(board: Board<C, R, SIZE>, settings: SolveSettings) -> Self {
        Self {
            results: Default::default(),
            queue: VecDeque::from(vec![Default::default()]),
            board,
            settings,
        }
    }

    fn add_to_queue(&mut self, path: Path<C, R, SIZE>) {
        if let Some(last) = path.tiles.last() {
            for adjacent in last.iter_adjacent().filter(|x| !path.used.get_bit(x)) {
                let mut new_path = path.clone();
                new_path.tiles.push(adjacent);
                new_path.used.set_bit(&adjacent, true);
                self.queue.push_back(new_path);
            }
        } else {
            for tile in Tile::iter_by_row() {
                let mut tiles: ArrayVec<[Tile<C, R>; SIZE]> = Default::default();
                let mut used: TileSet16<C, R, SIZE> = Default::default();
                tiles.push(tile);
                used.set_bit(&tile, true);

                self.queue.push_back(Path { tiles, used });
            }
        }
    }
}

impl<const C: u8, const R: u8, const SIZE: usize> Iterator for SolutionIter<C, R, SIZE> {
    type Item = FoundWord<C, R, SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.queue.pop_front() {
            let check_result = self.board.check(&path.tiles);

            match check_result {
                Ok(i) => {
                    self.add_to_queue(path.clone());
                    let should_return = self.settings.allow(i) && self.results.insert(i);

                    if should_return {
                        let found_word = FoundWord {
                            result: i,
                            path: path.tiles,
                        };
                        return Some(found_word);
                    }
                }
                Err(ParseFail::PartialSuccess) => {
                    self.add_to_queue(path);
                }
                Err(ParseFail::Failure) => {}
            }
        }

        None
    }
}
