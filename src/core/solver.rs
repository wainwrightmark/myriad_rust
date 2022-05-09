use crate::core::parser::ParseFail;
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

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Node {
    pub letter: Letter,
    pub coordinate: Coordinate,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}", self.letter, self.coordinate)
    }
}

impl Solver {
    pub fn check(&self, nodes: &Vec<Node>) -> Result<i32, ParseFail> {
        // let text: String = nodes
        //     .iter()
        //     .map(|x| x.letter.to_string())
        //     .collect::<Vec<String>>()
        //     .join("");
        let mut input = nodes     .iter()
        .map(|x| x.letter).peekable();
        

        crate::core::parser::parse_and_evaluate(&mut input)
    }

    pub fn get_possible_solutions(&self, board: &Board) -> impl Iterator<Item = FoundWord> {
        let mut results = HashMap::<i32, FoundWord>::new();
        let mut queue = VecDeque::<Vec<Node>>::new();
        let max_coordinate = board.max_coordinate();

        fn check(
            nodes: Vec<Node>,
            solver: &Solver,
            queue: &mut VecDeque<Vec<Node>>,
            results: &mut HashMap<i32, FoundWord>,
        ) {
            let check_result = solver.check(&nodes);

            match check_result {
                Ok(i) => {

                    if solver.settings.allow(i){
                        let found_word = FoundWord {
                            result: i,
                            path: nodes.iter().map(|x| x.coordinate).collect_vec(),
                        };
    
                        results.insert(i, found_word);
                    }

                    
                    queue.push_back(nodes);
                }
                Err(ParseFail::PartialSuccess) => {
                    queue.push_back(nodes);
                }
                Err(ParseFail::Failure) => {}
            }
        }

        for coordinate in board.max_coordinate().get_positions_up_to() {
            let letter = board.get_letter_at_coordinate(&coordinate);
            let node = Node { letter, coordinate };
            let nodes = vec![node];

            check(nodes, self, &mut queue, &mut results)
        }

        while !queue.is_empty() {
            let nodes = queue.pop_front().unwrap();
            let c = nodes.last().unwrap();
            let coordinates: HashSet<Coordinate> =
                HashSet::from_iter(nodes.iter().rev().map(|x| x.coordinate));

            for adjacent in c.coordinate.get_adjacent_positions(&max_coordinate) {
                if coordinates.contains(&adjacent) {
                    continue;
                }

                let letter = board.get_letter_at_coordinate(&adjacent);
                let new_node = Node {
                    letter,
                    coordinate: adjacent,
                };
                let mut new_nodes = nodes.clone();
                new_nodes.push(new_node);

                check(new_nodes, self, &mut queue, &mut results)
            }
        }

        results.into_values()
    }
}
