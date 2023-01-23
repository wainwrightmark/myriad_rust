use std::ops::Index;
use std::ops::IndexMut;

use crate::parser::*;
use crate::prelude::*;
pub use geometrid::prelude8::*;
use itertools::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Hash, Clone, Serialize, Deserialize, PartialOrd, Ord, Default)]
pub struct Board<const C: u8, const R: u8, const SIZE: usize>(pub Grid8<Rune, C, R, SIZE>);

static_assertions::assert_eq_size!(Board<3,3, 9>, [u8;9]);

impl<const C: u8, const R: u8, const SIZE: usize> Index<PointAbsolute8<C, R>>
    for Board<C, R, SIZE>
{
    type Output = Rune;

    fn index(&self, index: PointAbsolute8<C, R>) -> &Self::Output {
        &self.0[index]
    }
}

impl<const C: u8, const R: u8, const SIZE: usize> IndexMut<PointAbsolute8<C, R>>
    for Board<C, R, SIZE>
{
    fn index_mut(&mut self, index: PointAbsolute8<C, R>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const C: u8, const R: u8, const SIZE: usize> std::fmt::Display for Board<C, R, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const C: u8, const R: u8, const SIZE: usize> Board<C, R, SIZE> {
    pub fn check(&self, nodes: &[PointAbsolute8<C, R>]) -> Result<i32, ParseFail> {
        let mut input = nodes.iter().map(|x| self[*x]).peekable();

        crate::parser::parse_and_evaluate(&mut input)
    }

    pub fn try_create(letters: &str) -> Option<Board<C, R, SIZE>> {
        let r: Result<Vec<Rune>, _> = letters.chars().map(Rune::try_from).collect();

        match r {
            Err(_) => None,
            Ok(vector) => {
                let letters: [Rune; SIZE] = vector
                    .into_iter()
                    .pad_using(SIZE, |_| Rune::Blank)
                    .collect_vec()
                    .try_into()
                    .unwrap();

                Some(Self(Grid8(letters)))
            }
        }
    }

    pub fn get_word_text(&self, ps: &[PointAbsolute8<C, R>]) -> String {
        let word = ps
            .iter()
            .map(|c| {
                let rune = &self[*c];
                rune.to_string()
            })
            .join("");
        word
    }

    pub fn to_multiline_string(&self) -> String {
        let mut s = String::with_capacity(SIZE + (R as usize));

        for row in 0..R {
            if row != 0 {
                s.push_str("\r\n")
            };
            for column in 0..C {
                let p = PointAbsolute8::<C, R>::try_new(column, row).unwrap();
                let l = self[p].to_string();

                s.push_str(&l);
            }
        }

        s
    }

    pub fn to_single_string(&self) -> String {
        let mut s = String::with_capacity(SIZE + R as usize);
        for column in 0..C {
            for row in 0..R {
                let p = PointAbsolute8::<C, R>::try_new(column, row).unwrap();
                let l = self[p].to_string();

                s.push_str(&l);
            }
        }

        s
    }

    ///Flip along the vertical axis
    pub fn flip_vertical(&mut self) {
        self.0.flip_vertical()
    }

    ///Flip along the horizontal axis
    pub fn flip_horizontal(&mut self) {
        self.0.flip_horizontal()
    }
}

impl<const L: u8, const SIZE: usize> Board<L, L, SIZE> {
    pub fn rotate(&mut self) {
        self.0.rotate_clockwise()
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

    pub fn get_unique_string(&self) -> String {
        let mut options = (0..4)
            .into_iter()
            .cartesian_product(0..2)
            .map(|(rotate, reflect)| {
                PointAbsolute8::<L, L>::points_by_row()
                    .map(|c| c.rotate(rotate))
                    .map(|c| if reflect == 0 { c } else { c.flip_horizontal() })
                    .map(|c| self[c])
                    .join("")
            })
            .sorted();

        options.next().unwrap()
    }
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
        let mut board = Board::<3, 3, 9>::try_create(input).unwrap();

        if flip {
            board.flip_vertical();
        }

        for _ in 0..rotate {
            board.rotate();
        }

        let s = board.to_multiline_string();
        let expected_multiline = Board::<3, 3, 9>::try_create(expected)
            .unwrap()
            .to_multiline_string();

        assert_eq!(s, expected_multiline)
    }

    #[test_case("123456789")]
    fn test_is_canonical(input: &str) {
        let board = Board::<3, 3, 9>::try_create(input).unwrap();

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
