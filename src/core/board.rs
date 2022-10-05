use std::vec;

use crate::core::parser::*;
use crate::core::prelude::*;
use itertools::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Board<const COLUMNS: usize, const ROWS: usize> {
    #[serde_as(as = "[[_; COLUMNS]; ROWS]")]
    pub runes: [[Rune; COLUMNS]; ROWS],
}

impl Default for Board<3, 3> {
    fn default() -> Self {
        use Rune::*;
        Self {
            runes: [[Zero, Zero, Zero], [Zero, Zero, Zero], [Zero, Zero, Zero]],
        }
    }
}

static_assertions::assert_eq_size!(Board<3,3>, [u8;9]);

impl<const C: usize, const R: usize> std::fmt::Display for Board<C, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_multiline_string())
    }
}

impl<const C: usize, const R: usize> Board<C, R> {
    pub fn check(&self, nodes: &[Coordinate]) -> Result<i32, ParseFail> {
        let mut input = nodes
            .iter()
            .map(|x| self.get_letter_at_coordinate(x))
            .peekable();

        crate::core::parser::parse_and_evaluate(&mut input)
    }

    pub fn try_create(letters: &str) -> Option<Board<C, R>> {
        let r: Result<Vec<Rune>, _> = letters.chars().map(Rune::try_from).collect();

        match r {
            Err(_) => None,
            Ok(vector) => {
                //let len = vector.len();

                let letters: [[Rune; C]; R] = vector
                    .into_iter()
                    .pad_using(R * C, |_| Rune::Blank)
                    .chunks(R)
                    .into_iter()
                    .map(|x| {
                        let a: [Rune; C] = x.collect::<Vec<Rune>>().try_into().unwrap();
                        a
                    })
                    .collect::<Vec<[Rune; C]>>()
                    .try_into()
                    .unwrap();

                Some(Board { runes: letters })
            }
        }
    }

    pub fn get_word_text(&self, coordinates: &[Coordinate]) -> String {
        let word = coordinates
            .iter()
            .map(|c| {
                let letter = &self.get_letter_at_coordinate(c);

                letter.to_string()
            })
            .join("");
        word
    }

    pub fn with_set_letter(&self, letter: Rune, index: usize) -> Board<C, R> {
        let r = index % R;
        let c = index / R;

        let mut new_letters = self.runes;
        new_letters[r][c] = letter;

        Board { runes: new_letters }
    }

    pub fn get_unique_string(&self) -> String {
        if R != C {
            return format!("{}_{}", C, self.runes.iter().flatten().join(""));
        }

        let mut options = (0..4)
            .into_iter()
            .cartesian_product(0..2)
            .map(|(rotate, reflect)| {
                Coordinate::get_positions_up_to::<C, R>()
                    .map(|c| c.rotate_and_flip::<C, R>(rotate, reflect == 0))
                    .map(|c| self.get_letter_at_coordinate(&c))
                    .join("")
            })
            .sorted();

        options.next().unwrap()
    }

    pub const fn get_letter_at_coordinate(&self, coordinate: &Coordinate) -> Rune {
        self.runes[coordinate.column][coordinate.row]
    }

    pub const fn get_letter_at_index(&self, index: usize) -> Rune {
        self.runes[index % R][index / R]
    }

    pub fn to_multiline_string(&self) -> String {
        let mut s = String::with_capacity(self.runes.len() + R);
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
        let mut s = String::with_capacity(self.runes.len() + R as usize);
        for column in 0..C {
            for row in 0..R {
                let coordinate = Coordinate { row, column };
                let l = self.get_letter_at_coordinate(&coordinate).to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn flip(&mut self) {
        for row in 0..(R / 2) {
            for column in 0..C {
                let o_row = R - (1 + row);
                let swap = self.runes[row][column];
                self.runes[row][column] = self.runes[o_row][column];
                self.runes[o_row][column] = swap;
            }
        }
    }

    pub fn rotate(&mut self) {
        if R != C {
            panic!("Cannot rotate uneven board");
        }
        for row in 0..=(R / 2) {
            for column in row..=(C / 2) {
                let o_row = R - (1 + row);
                let o_column = C - (1 + column);
                if row != o_row || column != o_column {
                    let swap = self.runes[row][column];
                    self.runes[row][column] = self.runes[o_row][column];
                    self.runes[o_row][column] = self.runes[o_row][o_column];
                    self.runes[o_row][o_column] = self.runes[row][o_column];
                    self.runes[row][o_column] = swap;
                }
            }
        }
    }

    pub fn is_canonical_form(&mut self) -> bool {
        let mut o = self.clone();
        for _ in 0..3 {
            o.flip();
            if self > &mut o {
                return false;
            }
            o.rotate();
            if self > &mut o {
                return false;
            }
        }
        o.flip();
        if self > &mut o {
            return false;
        }
        //rotating again will return back to self

        return true;
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

        let mut strings = vec![
            self.to_single_string(),
            one_thousand_result,
            ten_thousand_result,
        ];

        let mut nums = 0;
        let mut operators = 0;
        let mut blanks = 0;
        for rune in self.runes.iter().flatten() {
            let rt: RuneType = RuneType::from(*rune);

            match rt {
                RuneType::Digit => nums += 1,
                RuneType::Operator => operators += 1,
                RuneType::Blank => blanks += 1,
            }
        }

        strings.push(nums.to_string());
        strings.push(operators.to_string());
        strings.push(blanks.to_string());

        let legal_letters = ClassicGameMode {}.legal_letters();

        for l in legal_letters {
            let c = self.runes.iter().flatten().filter(|&x| x == l).count();
            strings.push(c.to_string());
        }

        strings.join(" ")
    }
}
