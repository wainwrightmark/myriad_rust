use crate::core::{parser::ParseFail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::core::prelude::*;
use itertools::Itertools;

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SolveSettings {
    pub min: i32,
    pub max: i32,
}

impl SolveSettings {
    pub fn allow(&self, num: i32) -> bool {
        self.min <= num && num <= self.max
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

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Solver {
    pub settings: SolveSettings,
}



impl Solver {
    

    pub fn get_possible_solutions(&self, board: &Board) -> impl Iterator<Item = FoundWord> {
        let mut results = HashMap::<i32, FoundWord>::new();
        let mut queue = VecDeque::<Vec<Coordinate>>::new();
        let max_coordinate = board.max_coordinate();

        

        fn check(
            coordinates: Vec<Coordinate>,
            solver: &Solver,
            queue: &mut VecDeque<Vec<Coordinate>>,
            results: &mut HashMap<i32, FoundWord>,
            board: &Board
        ) {
            let check_result = board.check(&coordinates);

            match check_result {
                Ok(i) => {

                    if solver.settings.allow(i){
                        let found_word = FoundWord {
                            result: i,
                            path: coordinates.clone(),
                        };
    
                        results.insert(i, found_word);
                    }

                    
                    queue.push_back(coordinates);
                }
                Err(ParseFail::PartialSuccess) => {
                    queue.push_back(coordinates);
                }
                Err(ParseFail::Failure) => {}
            }
        }

        for coordinate in board.max_coordinate().get_positions_up_to() {                        
            let coordinates = vec![coordinate];
            check(coordinates, self, &mut queue, &mut results, board)
        }

        while !queue.is_empty() {
            let nodes = queue.pop_front().unwrap();
            let c = nodes.last().unwrap();
            let coordinates: HashSet<Coordinate> =
                HashSet::from_iter(nodes.iter().rev().map(|&x| x));

            for adjacent in c.get_adjacent_positions(&max_coordinate) {
                if coordinates.contains(&adjacent) {
                    continue;
                }                
                
                let mut new_nodes = nodes.clone();
                new_nodes.push(adjacent);

                check(new_nodes, self, &mut queue, &mut results, board)
            }
        }

        results.into_values()
    }
}
