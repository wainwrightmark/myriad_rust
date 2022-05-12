use crate::core::parser::ParseFail;
use crate::core::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};


#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
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

    pub fn solve(self, board: Board) -> impl Iterator<Item = FoundWord>{
        let solution_iter = SolutionIter::new(board, self);
        solution_iter
    }    
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self { min: 1, max: 100 }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct FoundWord {
    pub result: i32,
    pub path: Vec<Coordinate>,
}

impl std::fmt::Display for FoundWord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}", self.result, self.path.iter().join(""))
    }
}

struct SolutionIter {
    results: HashSet<i32>,
    settings: SolveSettings,
    queue: VecDeque<Vec<Coordinate>>,
    board: Board,
    max_coordinate: Coordinate,
}

impl SolutionIter {
    pub fn new(board: Board, settings: SolveSettings) -> Self {
        Self {
            results: Default::default(),
            max_coordinate: board.max_coordinate(),
            queue: VecDeque::from(vec![vec![]]),
            board,
            settings,
        }
    }

    fn add_to_queue(&mut self, coordinates: Vec<Coordinate>) {
        if let Some(last) = coordinates.last() {
            for adjacent in last
                .get_adjacent_positions(&self.max_coordinate)
                .filter(|x| !coordinates.contains(x))
            {
                let mut new_nodes = coordinates.clone();
                new_nodes.push(adjacent);
                self.queue.push_back(new_nodes);
            }
        } else {
            for coordinate in self.max_coordinate.get_positions_up_to() {
                let single_coordinate = vec![coordinate];
                self.queue.push_back(single_coordinate);
            }
        }
    }
}

impl Iterator for SolutionIter {
    type Item = FoundWord;

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
