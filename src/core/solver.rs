use crate::core::parser::ParseFail;
use crate::core::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::{Deserialize, Serialize};
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
    pub fn solve<const C: usize, const R: usize>(
        self,
        board: Board<C, R>,
    ) -> impl Iterator<Item = FoundWord<C, R>>
    where
        [(); C * R]:,
    {
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
pub struct FoundWord<const COLUMNS: usize, const ROWS: usize> {
    pub result: i32,
    pub path: Vec<Coordinate<COLUMNS, ROWS>>,
}

impl<const C: usize, const R: usize> std::fmt::Display for FoundWord<C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.result, self.path.iter().join(""))
    }
}

struct SolutionIter<const COLUMNS: usize, const ROWS: usize>
where
    [(); COLUMNS * ROWS]:,
{
    results: HashSet<i32>,
    settings: SolveSettings,
    queue: VecDeque<Vec<Coordinate<COLUMNS, ROWS>>>,
    board: Board<COLUMNS, ROWS>,
}

impl<const C: usize, const R: usize> SolutionIter<C, R>
where
    [(); C * R]:,
{
    pub fn new(board: Board<C, R>, settings: SolveSettings) -> Self {
        Self {
            results: Default::default(),
            queue: VecDeque::from(vec![vec![]]),
            board,
            settings,
        }
    }

    fn add_to_queue(&mut self, coordinates: Vec<Coordinate<C, R>>) {
        if let Some(last) = coordinates.last() {
            for adjacent in last
                .get_adjacent_positions()
                .filter(|x| !coordinates.contains(x))
            {
                let mut new_nodes = coordinates.clone();
                new_nodes.push(adjacent);
                self.queue.push_back(new_nodes);
            }
        } else {
            for coordinate in Coordinate::get_positions_up_to() {
                let single_coordinate = vec![coordinate];
                self.queue.push_back(single_coordinate);
            }
        }
    }
}

impl<const C: usize, const R: usize> Iterator for SolutionIter<C, R>
where
    [(); C * R]:,
{
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
