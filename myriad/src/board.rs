use std::ops::Index;
use std::ops::IndexMut;

use crate::parser::*;
use crate::prelude::*;
use itertools::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Board<const COLUMNS: usize, const ROWS: usize>
where
    [(); COLUMNS * ROWS]:,
{
    #[serde_as(as = "[_; COLUMNS * ROWS]")]
    pub runes: [Rune; COLUMNS * ROWS],
}

impl<const C: usize, const R: usize> Default for Board<C, R>
where
    [(); C * R]:,
{
    fn default() -> Self {
        use Rune::*;
        Self {
            runes: [Zero; C * R],
        }
    }
}

static_assertions::assert_eq_size!(Board<3,3>, [u8;9]);

impl<const C: usize, const R: usize> Index<Coordinate<C, R>> for Board<C, R>
where
    [(); C * R]:,
{
    type Output = Rune;

    fn index(&self, index: Coordinate<C, R>) -> &Self::Output {
        &self.runes[index.0 as usize]
    }
}

impl<const C: usize, const R: usize> IndexMut<Coordinate<C, R>> for Board<C, R>
where
    [(); C * R]:,
{
    fn index_mut(&mut self, index: Coordinate<C, R>) -> &mut Self::Output {
        &mut self.runes[index.0 as usize]
    }
}

impl<const C: usize, const R: usize> std::fmt::Display for Board<C, R>
where
    [(); C * R]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_multiline_string())
    }
}

impl<const C: usize, const R: usize> Board<C, R>
where
    [(); C * R]:,
{
    pub fn check(&self, nodes: &[Coordinate<C, R>]) -> Result<i32, ParseFail> {
        let mut input = nodes.iter().map(|x| self[*x]).peekable();

        crate::parser::parse_and_evaluate(&mut input)
    }

    pub fn try_create(letters: &str) -> Option<Board<C, R>> {
        let r: Result<Vec<Rune>, _> = letters.chars().map(Rune::try_from).collect();

        match r {
            Err(_) => None,
            Ok(vector) => {
                let letters: [Rune; C * R] = vector
                    .into_iter()
                    .pad_using(C * R, |_| Rune::Blank)
                    .collect_vec()
                    .try_into()
                    .unwrap();

                Some(Board { runes: letters })
            }
        }
    }

    pub fn get_word_text(&self, coordinates: &[Coordinate<C, R>]) -> String {
        let word = coordinates
            .iter()
            .map(|c| {
                let rune = &self[*c];
                rune.to_string()
            })
            .join("");
        word
    }

    pub fn get_unique_string(&self) -> String {
        //TODO improve
        if R != C {
            return format!("{}_{}", C, self.runes.iter().join(""));
        }

        let mut options = (0..4)
            .into_iter()
            .cartesian_product(0..2)
            .map(|(rotate, reflect)| {
                Coordinate::<C, R>::get_positions_up_to()
                    .map(|c| c.rotate_and_flip(rotate, reflect == 0))
                    .map(|c| self[c])
                    .join("")
            })
            .sorted();

        options.next().unwrap()
    }

    pub fn to_multiline_string(&self) -> String {
        let mut s = String::with_capacity(self.runes.len() + R);

        for row in 0..R {
            if row != 0 {
                s.push_str("\r\n")
            };
            for column in 0..C {
                let coordinate = Coordinate::<C, R>::create(column, row);
                let l = self[coordinate].to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn to_single_string(&self) -> String {
        let mut s = String::with_capacity(self.runes.len() + R as usize);
        for column in 0..C {
            for row in 0..R {
                let coordinate = Coordinate::<C, R>::create(column, row);
                let l = self[coordinate].to_string();

                s.push_str(&l);
            }
        }

        s
    }

    ///Flip along the vertical axis
    pub fn flip_vertical(&mut self) {
        for column in 0..(C / 2) {
            for row in 0..R {
                let coordinate = Coordinate::create(column, row);
                let o_coordinate = Coordinate::create(C - (1 + column), row);

                let swap = self[coordinate];
                self[coordinate] = self[o_coordinate];
                self[o_coordinate] = swap;
            }
        }
    }

    ///Flip along the horizontal axis
    pub fn flip_horizontal(&mut self) {
        for row in 0..(R / 2) {
            for column in 0..C {
                let coordinate = Coordinate::create(column, row);
                let o_coordinate = Coordinate::create(column, R - (1 + row));

                let swap = self[coordinate];
                self[coordinate] = self[o_coordinate];
                self[o_coordinate] = swap;
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
                    let coordinate0 = Coordinate::create(column, row);
                    let coordinate1 = Coordinate::create(row, o_column);
                    let coordinate2 = Coordinate::create(o_column, o_row);
                    let coordinate3 = Coordinate::create(o_row, column);

                    let swap = self[coordinate0];
                    self[coordinate0] = self[coordinate1];
                    self[coordinate1] = self[coordinate2];
                    self[coordinate2] = self[coordinate3];
                    self[coordinate3] = swap;
                }
            }
        }
    }

    pub fn is_canonical_form(&self) -> bool {
        let mut o = self.clone();
        for _ in 0..3 {
            o.flip_vertical();
            if self > &mut o {
                return false;
            }
            o.rotate();
            if self > &mut o {
                return false;
            }
        }
        o.flip_vertical();
        if self > &mut o {
            return false;
        }
        //rotating again will return back to self

        true
    }

    // pub fn get_board_data(&self) -> String {
    //     let one_thousand_solve_settings = SolveSettings { min: 1, max: 1000 };
    //     let ten_thousand_solve_settings = SolveSettings { min: 1, max: 10000 };

    //     let one_thousand_result = one_thousand_solve_settings
    //         .solve(self.clone())
    //         .count()
    //         .to_string();
    //     let ten_thousand_result = ten_thousand_solve_settings
    //         .solve(self.clone())
    //         .count()
    //         .to_string();

    //     let mut strings = vec![
    //         self.to_single_string(),
    //         one_thousand_result,
    //         ten_thousand_result,
    //     ];

    //     let mut nums = 0;
    //     let mut operators = 0;
    //     let mut blanks = 0;
    //     let mut numerals = 0;

    //     for rune in self.runes {
    //         let rt: RuneType = RuneType::from(rune);

    //         match rt {
    //             RuneType::Digit => nums += 1,
    //             RuneType::Operator => operators += 1,
    //             RuneType::Blank => blanks += 1,
    //             RuneType::RomanNumeral => numerals += 1,
    //         }
    //     }

    //     strings.push(nums.to_string());
    //     strings.push(operators.to_string());
    //     strings.push(blanks.to_string());
    //     strings.push(numerals.to_string());

    //     let legal_letters = ClassicGameMode {}.legal_letters();

    //     for l in legal_letters {
    //         let c = self.runes.iter().filter(|&x| x == l).count();
    //         strings.push(c.to_string());
    //     }

    //     strings.join(" ")
    // }
}

#[cfg(test)]
mod tests {
    use ntest::test_case;

    use super::Board;

    #[test_case("123456789", false, 0, "123456789")]
    #[test_case("123456789", true, 0, "789456123")]
    #[test_case("123456789", false, 1, "741852963")]
    #[test_case("123456789", false, 2, "987654321")]
    #[test_case("123456789", false, 3, "369258147")]
    #[test_case("123456789", false, 4, "123456789")]
    #[test_case("123456789", true, 1, "147258369")]
    #[test_case("123456789", true, 2, "321654987")]
    #[test_case("123456789", true, 3, "963852741")]
    fn test_board_flip_rotate(input: &str, flip: bool, rotate: i32, expected: &str) {
        let mut board = Board::<3, 3>::try_create(input).unwrap();

        if flip {
            board.flip_horizontal();
        }

        for _ in 0..rotate {
            board.rotate();
        }

        let s = board.to_multiline_string();
        let expected_multiline = Board::<3, 3>::try_create(expected)
            .unwrap()
            .to_multiline_string();

        assert_eq!(s, expected_multiline)
    }

    #[test_case("123456789")]
    fn test_is_canonical(input: &str) {
        let board = Board::<3, 3>::try_create(input).unwrap();

        assert!(board.is_canonical_form());

        let mut o = board.clone();
        for _ in 0..3 {
            o.flip_vertical();
            if board > o {
                assert!(!board.is_canonical_form());
            }
            o.rotate();
            if board > o {
                assert!(!board.is_canonical_form());
            }
        }
        o.flip_vertical();
        if board > o {
            assert!(!board.is_canonical_form());
        }
    }
}
