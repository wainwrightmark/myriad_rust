use crate::core::parser::*;
use crate::core::prelude::*;
use itertools::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Board {
    pub columns: u8,
    pub letters: Vec<Letter>,
}

impl Default for Board {
    fn default() -> Self {
        Board::try_create("-+718325+").unwrap()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_multiline_string())
    }
}

impl Board {
    pub fn check(&self, nodes: &Vec<Coordinate>) -> Result<i32, ParseFail> {
        let mut input = nodes
            .iter()
            .map(|x| self.get_letter_at_coordinate(x))
            .peekable();

        crate::core::parser::parse_and_evaluate(&mut input)
    }

    pub fn try_create(letters: &str) -> Option<Board> {
        let r: Option<Vec<Letter>> = letters.chars().map(Letter::try_create).collect();

        match r {
            None => None,
            Some(vector) => {
                let len = vector.len();

                let max_co = Coordinate::get_max_coordinate_for_square_grid(len as u8);

                Some(Board {
                    columns: max_co.column + 1,
                    letters: vector
                        .into_iter()
                        .pad_using(((max_co.row + 1) * (max_co.column + 1)) as usize, |_| {
                            Letter::Blank
                        })
                        .collect(),
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

    pub fn with_set_letter(&self, letter: Letter, index: usize) -> Board {
        let mut new_letters = self.letters.clone();
        new_letters[index] = letter;

        Board {
            letters: new_letters,
            columns: self.columns,
        }
    }

    pub fn get_unique_string(&self) -> String {
        if self.columns != self.rows() {
            return format!("{}_{}", self.columns, self.letters.iter().join(""));
        }

        let max = self.max_coordinate();
        let mut options = (0..4)
            .into_iter()
            .cartesian_product(0..2)
            .map(|(rotate, reflect)| {
                self.max_coordinate()
                    .get_positions_up_to()
                    .map(|c| c.rotate_and_flip(max, rotate, reflect == 0))
                    .map(|c| self.get_letter_at_coordinate(&c))
                    .join("")
            })
            .sorted();

        options.next().unwrap()
    }

    pub fn rows(&self) -> u8 {
        self.letters.len() as u8 / self.columns
    }

    pub fn max_coordinate(&self) -> Coordinate {
        let column = self.columns - 1;
        let row = self.rows() - 1;

        Coordinate { row, column }
    }

    pub fn get_letter_at_coordinate(&self, coordinate: &Coordinate) -> Letter {
        let index: usize = ((coordinate.row * self.columns) + coordinate.column) as usize;
        self.get_letter_at_index(index)
    }

    pub fn get_letter_at_index(&self, index: usize) -> Letter {
        self.letters[index % self.letters.len()]
    }

    pub fn to_multiline_string(&self) -> String {
        let mut s = String::with_capacity(self.letters.len() + self.rows() as usize);
        for column in 0..self.columns {
            if column != 0 {
                s.push_str("\r\n")
            };
            for row in 0..self.rows() {
                let coordinate = Coordinate { row, column };
                let l = self.get_letter_at_coordinate(&coordinate).to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn to_single_string(&self) -> String {
        let mut s = String::with_capacity(self.letters.len() + self.rows() as usize);
        for column in 0..self.columns {
            for row in 0..self.rows() {
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

        let mut strings = Vec::<String>::new();

        strings.push(self.to_single_string());
        strings.push(
            one_thousand_solve_settings
                .solve(self.clone())
                .count()
                .to_string(),
        );
        strings.push(
            ten_thousand_solve_settings
                .solve(self.clone())
                .count()
                .to_string(),
        );

        let mut nums = 0;
        let mut operators = 0;
        let mut blanks = 0;
        for letter in self.letters.iter() {
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
            let c = self.letters.iter().filter(|&x| x == &l).count();
            strings.push(c.to_string());
        }

        strings.join(" ")
    }
}
