use std::vec;

use crate::core::parser::*;
use crate::core::prelude::*;
use itertools::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]

#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Board<const COLUMNS: usize, const ROWS: usize> {    
        
    #[serde_as(as = "[[_; COLUMNS]; ROWS]")]
    pub letters: [[Letter; COLUMNS]; ROWS],
}

impl Default for Board<3,3> {
    fn default() -> Self {
        Board::try_create("-+718325+").unwrap()
    }
}

impl<const C: usize,const R: usize> std::fmt::Display for Board<C,R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_multiline_string())
    }
}

impl<const C: usize,const R: usize> Board<C,R> {
    pub fn check(&self, nodes: &[Coordinate]) -> Result<i32, ParseFail> {
        let mut input = nodes
            .iter()
            .map(|x| self.get_letter_at_coordinate(x))
            .peekable();

        crate::core::parser::parse_and_evaluate(&mut input)
    }

    pub fn try_create(letters: &str) -> Option<Board<C,R>> {
        let r: Option<Vec<Letter>> = letters.chars().map(Letter::try_create).collect();

        match r {
            None => None,
            Some(vector) => {
                //let len = vector.len();

                let letters :  [[Letter; C]; R] = vector.into_iter()
                .pad_using(R * C, |_| {
                            Letter::Blank
                        })


                .chunks(R).into_iter().map(|x|
                    {
                    let a: [Letter; C] = x.collect::<Vec<Letter>>().try_into().unwrap();
                    return a;
                    }
                    ).collect::<Vec<[Letter; C]>>().try_into().unwrap();

                //let max_co = Coordinate::get_max_coordinate_for_square_grid(len as u8);

                Some(Board {
                    letters
                    // letters: vector
                    //     .into_iter()
                    //     .pad_using(((max_co.row + 1) * (max_co.column + 1)) as usize, |_| {
                    //         Letter::Blank
                    //     })
                    //     .collect(),
                })
            }
        }
    }

    pub fn get_word_text(&self, coordinates: &[Coordinate]) -> String {
        let word = coordinates
            .iter()
            .map(|c| {
                let letter = &self.get_letter_at_coordinate(c);

                letter.word_text()
            })
            .join("");
        word
    }

    pub fn with_set_letter(&self, letter: Letter, index: usize) -> Board<C,R> {
        let r = index % R;
        let c = index / R;

        let mut new_letters = self.letters.clone();
        new_letters[r][c] = letter;

        Board {
            letters: new_letters,
        }
    }

    pub fn get_unique_string(&self) -> String {
        if R != C {
            return format!("{}_{}", C, self.letters.iter().flat_map(|x|x) .join(""));
        }

        
        let mut options = (0..4)
            .into_iter()
            .cartesian_product(0..2)
            .map(|(rotate, reflect)| {
                Coordinate::get_positions_up_to::<C, R>()
                    .map(|c| c.rotate_and_flip::<C, R>( rotate, reflect == 0))
                    .map(|c| self.get_letter_at_coordinate(&c))
                    .join("")
            })
            .sorted();

        options.next().unwrap()
    }



    pub fn get_letter_at_coordinate(&self, coordinate: &Coordinate) -> Letter {

        self.letters[coordinate.column ][coordinate.row ]
    }

    pub fn get_letter_at_index(&self, index: usize) -> Letter {
        self.letters[index % R][index / R]
    }

    pub fn to_multiline_string(&self) -> String {
        let mut s = String::with_capacity(self.letters.len() + R);
        for column in 0..C {
            if column != 0 {
                s.push_str("\r\n")
            };
            for row in 0..R {
                let coordinate = Coordinate { row, column };
                let l = self.get_letter_at_coordinate(&coordinate).to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn to_single_string(&self) -> String {
        let mut s = String::with_capacity(self.letters.len() + R as usize);
        for column in 0..C {
            for row in 0..R {
                let coordinate = Coordinate { row, column };
                let l = self.get_letter_at_coordinate(&coordinate).to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn get_board_data(&self) -> String {
        let one_thousand_solve_settings = SolveSettings { min: 1, max: 1000 };
        let ten_thousand_solve_settings = SolveSettings { min: 1, max: 10000 };

        let one_thousand_result = one_thousand_solve_settings
        .solve(self.clone())
        .count()
        .to_string();
        let ten_thousand_result = ten_thousand_solve_settings
        .solve(self.clone())
        .count()
        .to_string();


        let mut strings = vec![self.to_single_string(), one_thousand_result, ten_thousand_result];

        let mut nums = 0;
        let mut operators = 0;
        let mut blanks = 0;
        for letter in self.letters.iter().flat_map(|x|x) {
            match letter {
                Letter::Number { value: _ } => nums += 1,
                Letter::Operator { operation: _ } => operators += 1,
                Letter::Blank => blanks += 1,
            }
        }

        strings.push(nums.to_string());
        strings.push(operators.to_string());
        strings.push(blanks.to_string());

        for l in Letter::legal_letters() {
            let c = self.letters.iter().flat_map(|x|x).filter(|&x| x == &l).count();
            strings.push(c.to_string());
        }

        strings.join(" ")
    }
}
