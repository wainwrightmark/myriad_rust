use crate::parser::ParseFail;
use crate::prelude::*;
use geometrid::prelude::Tile;
use itertools::Itertools;
use num::ToPrimitive;
use serde::{Deserialize, Serialize};
use tinyvec::ArrayVec;
use std::collections::{HashSet, VecDeque};

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
    ) -> impl Iterator<Item = FoundWord<C, R>> {
        SolutionIter::new(board, self)
    }

    pub fn total_solutions(&self) -> usize {
        (self.max - self.min + 1).to_usize().unwrap()
    }
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self { min: 1, max: 100 }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct FoundWord<const C: u8, const R: u8> {
    pub result: i32,
    pub path: ArrayVec<[Tile<C, R>;9]>,
}

impl<const C: u8, const R: u8> std::fmt::Display for FoundWord<C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.result, self.path.iter().join(""))
    }
}

struct SolutionIter<const C: u8, const R: u8, const SIZE: usize> {
    results: HashSet<i32>,
    settings: SolveSettings,
    queue: VecDeque<ArrayVec<[Tile<C, R>;9]>>,
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

    fn add_to_queue(&mut self, coordinates: ArrayVec<[Tile<C, R>;9]>) {
        if let Some(last) = coordinates.last() {
            for adjacent in last
                .iter_adjacent()
                .filter(|x| !coordinates.contains(x))
            {
                let mut new_nodes = coordinates.clone();
                new_nodes.push(adjacent);
                self.queue.push_back(new_nodes);
            }
        } else {
            for coordinate in Tile::iter_by_row() {
                let mut path: ArrayVec<[Tile<C, R>;9]> = Default::default();
                path.push(coordinate);

                self.queue.push_back(path);
            }
        }
    }
}

impl<const C: u8, const R: u8, const SIZE: usize> Iterator for SolutionIter<C, R, SIZE> {
    type Item = FoundWord<C, R>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(coordinates) = self.queue.pop_front() {
            let check_result = self.board.check(&coordinates);

            match check_result {
                Ok(i) => {
                    self.add_to_queue(coordinates.clone());
                    let should_return = self.settings.allow(i) && self.results.insert(i);

                    if should_return {
                        let found_word = FoundWord {
                            result: i,
                            path: coordinates,
                        };
                        return Some(found_word);
                    }
                }
                Err(ParseFail::PartialSuccess) => {
                    self.add_to_queue(coordinates);
                }
                Err(ParseFail::Failure) => {}
            }
        }

        None
    }
}
