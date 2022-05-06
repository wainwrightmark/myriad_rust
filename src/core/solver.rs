use std::collections::{HashMap, VecDeque, HashSet};
use serde::{Serialize, Deserialize};

use im::vector::Vector;
use im::vector;
use itertools::Itertools;
use crate::core:: prelude::*;

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SolveSettings{
    pub min: i32,
    pub max: i32
}

impl SolveSettings{
    pub fn allow(&self, num:i32) -> bool {
        self.min <= num && num <= self.max
    }
}

impl Default for SolveSettings{
    fn default() -> Self {
        Self { min: 1, max: 100 }
    }
}

#[derive(PartialEq, Eq, Debug, Clone,Serialize, Deserialize)]
pub struct FoundWord{
    pub result: i32,
    pub path: Vec<Coordinate>
}

impl std::fmt::Display for FoundWord {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} = {}",self.result, self.path.iter().join(""))
    }
}



#[derive(PartialEq, Eq, Debug)]
pub enum WordCheckResult{
    Invalid,
    Legal{word: FoundWord},
    Illegal{word: FoundWord},

}

impl std::fmt::Display for WordCheckResult {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            WordCheckResult::Invalid => write!(f, "invalid"),
            WordCheckResult::Legal { word } => write!(f, "{}",word),
            WordCheckResult::Illegal { word } => write!(f, "illegal: {}",word)
        }
    }
}


#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Solver{
    pub settings: SolveSettings
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Node{
    pub letter: Letter,
    pub coordinate: Coordinate
}

impl std::fmt::Display for Node {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}",self.letter, self.coordinate)
    }
}

impl Solver {
    pub fn check(&self, nodes: &Vector<Node>) -> WordCheckResult{

        let text : String = nodes.iter().map(|x|x.letter .to_string()).collect::<Vec<String>>().join("");

        let parse_result = meval::eval_str(text);

        if let Ok(x) = parse_result{
            if x.round() == x && x < (i32::MAX as f64) && x > (i32::MIN as f64){
                let u = x as i32;
                let found_word = FoundWord{result: u, path: nodes.iter().map(|x|x.coordinate).collect_vec() };

                if self.settings.allow(u)
                {
                    return WordCheckResult::Legal{word: found_word}
                }
                else {return WordCheckResult::Illegal{word: found_word}}
            }
        }
        
        WordCheckResult::Invalid
    }

    pub fn is_legal_prefix(&self, nodes: &Vector<Node>) -> bool{
        if nodes.is_empty() {return true;}
        
        //Check first letter
        if let Letter::Operator {operation: letter_op} = nodes[0].letter{
            if letter_op != Operation::Minus{return false;}
        }

        //Check for blanks
        if nodes.iter().any(|f|f.letter == Letter::Blank) {return false;}

        //Check there are no pairs of operators (except where second is minus)
        for (a,b) in nodes.iter().tuple_windows(){
            if let Letter::Operator{operation: op_b} = b.letter{
                if op_b != Operation::Minus  {
                    if let Letter::Operator{operation: _} = a.letter{
                        return false;
                    }
                }
            }
        }

        true
    }
    

    pub fn get_possible_solutions(&self, board: &Board) -> impl Iterator<Item =FoundWord>{

        let mut results = HashMap::<i32, FoundWord>::new();
        let mut queue = VecDeque::<im::vector::Vector<Node>>::new();
        let max_coordinate = board.max_coordinate();

        fn check(nodes: Vector<Node>,solver: &Solver, queue: &mut VecDeque<Vector<Node>>, results: &mut HashMap<i32, FoundWord>){
            let check_result = solver.check(&nodes);

            if let WordCheckResult::Legal{word} = check_result{
                results.insert(word.result, word);
            }
            else if !solver.is_legal_prefix(&nodes){
                return;
            }

            queue.push_back(nodes);
        }


        for coordinate in board.max_coordinate().get_positions_up_to(){
            let letter = board.get_letter_at_coordinate(&coordinate);
            let node =Node{letter, coordinate };
            let nodes = vector![node ];

            check(nodes, self, &mut queue, &mut results)
        }

        while !queue.is_empty(){
            let nodes = queue.pop_front().unwrap();
            let c = nodes.back().unwrap();
            let coordinates: HashSet<Coordinate> = HashSet::from_iter(nodes.iter().rev().map(|x|x.coordinate)); 

            for adjacent in c.coordinate.get_adjacent_positions(&max_coordinate)
             {
                if coordinates.contains(&adjacent){continue;}

                let letter = board.get_letter_at_coordinate(&adjacent);
                let new_node = Node { letter, coordinate: adjacent };
                let mut new_nodes = nodes.clone();
                new_nodes.push_back(new_node);

                check(new_nodes, self, &mut queue, &mut results)
             }
        }

        
        results.into_values()
    }
}
