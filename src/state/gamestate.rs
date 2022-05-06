use itertools::Itertools;
use num::ToPrimitive;
use serde::Deserialize;
use serde::Serialize;

use crate::core::prelude::*;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Gamestate {
    pub board: Board,
    pub chosen_positions: Vec<Coordinate>,
    pub rotate: i8,
    pub flip: bool,
    pub solver: Solver,
}

impl Default for Gamestate {
    fn default() -> Self {
        let board = Board::try_create("78++25316").unwrap();

        Self {
            board,
            chosen_positions: Default::default(),
            rotate: Default::default(),
            flip: Default::default(),
            solver: Default::default(),
        }
    }
}

impl Gamestate {
    pub fn get_path_data(&self, square_size: f64) -> String {
        let coordinates = self.get_path_coordinates(square_size);

        let d = "M ".to_string()
            + &coordinates
                .iter()
                .map(|x| format!("{:.2} {:.2}", x.0, x.1))
                .join(" L ");

        d
    }

    fn get_path_coordinates(&self, square_size: f64) -> Vec<(f64, f64)> {
        fn get_inbetween(d1: f64, d2: f64, numerator: f64, denominator: f64) -> f64 {
            let t = d2 * numerator + d1 * (denominator - numerator);
            t / denominator
        }

        if !self.chosen_positions.is_empty() {
            let locations = self
                .chosen_positions
                .iter()
                .map(|x| self.get_location(x, square_size))
                .collect_vec();

            (0..self.board.letters.len())
                .map(|i| {
                    let index = (i * self.chosen_positions.len()) / self.board.letters.len();
                    let remainder = (i * self.chosen_positions.len()) % self.board.letters.len();

                    let loc = locations[index];

                    if remainder == 0 || locations.len() <= index + 1 {
                        loc
                    } else {
                        let next = locations[index + 1];

                        (
                            get_inbetween(
                                loc.0,
                                next.0,
                                remainder as f64,
                                self.board.letters.len() as f64,
                            ),
                            get_inbetween(
                                loc.1,
                                next.1,
                                remainder as f64,
                                self.board.letters.len() as f64,
                            ),
                        )
                    }
                })
                .collect_vec()
        } else {
            let centre = (
                square_size * self.board.columns.to_f64().unwrap() / 2.0,
                square_size * self.board.rows().to_f64().unwrap() / 2.0,
            );
            let zero_vec = vec![centre; self.board.columns as usize];
            zero_vec
        }
    }

    pub fn get_location(&self, coordinate: &Coordinate, square_size: f64) -> (f64, f64) {
        let rotated =
            coordinate.rotate_and_flip(self.board.max_coordinate(), self.rotate, self.flip);

        let cx = (rotated.column as f64 + 0.5) * square_size;
        let cy = (rotated.row as f64 + 0.5) * square_size;

        (cx, cy)
    }

    pub fn get_color(&self, coordinate: &Coordinate) -> &str {
        if self.chosen_positions.is_empty() {
            return "grey";
        }

        let move_result = self.get_move_result(coordinate);

        match move_result {
            MoveResult::WordComplete {
                word: _,
                coordinates: _,
            } => "darkgreen",
            MoveResult::WordContinued {
                word: _,
                coordinates: _,
            } => "green",
            MoveResult::WordAbandoned => "blue",
            MoveResult::MoveRetraced {
                word: _,
                coordinates: _,
            } => "lightgreen",
            MoveResult::IllegalMove => "grey",
        }
    }

    pub fn get_move_result(&self, coordinate: &Coordinate) -> MoveResult {
        if !self.chosen_positions.is_empty()
            && (self.chosen_positions.first().unwrap() == coordinate
                || self.chosen_positions.last().unwrap() == coordinate)
        {
            return MoveResult::WordAbandoned;
        }

        let find_result = self
            .chosen_positions
            .iter()
            .find_position(|z| z == &coordinate);

        if let Some((index, _)) = find_result {
            let new_chosen_positions: Vec<Coordinate> = self
                .chosen_positions
                .iter()
                .take(index + 1)
                .copied()
                .collect_vec();
            let word = self.get_word_text(&new_chosen_positions);
            return MoveResult::MoveRetraced {
                word,
                coordinates: new_chosen_positions,
            };
        }

        if self.chosen_positions.is_empty()
            || self
                .chosen_positions
                .last()
                .unwrap()
                .is_adjacent(coordinate)
        {
            let mut new_chosen_positions = self.chosen_positions.clone();
            new_chosen_positions.push(*coordinate);

            let word = self.get_word_text(&new_chosen_positions);

            let nodes_iter = new_chosen_positions.iter().map(|c| {
                let letter = &self.board.get_letter_at_coordinate(c);
                Node {
                    coordinate: *c,
                    letter: *letter,
                }
            });

            let nodes = im::Vector::from_iter(nodes_iter);
            let check_result = self.solver.check(&nodes);

            let final_result = match check_result {
                WordCheckResult::Invalid => {
                    if self.solver.is_legal_prefix(&nodes) {
                        MoveResult::WordContinued {
                            word,
                            coordinates: new_chosen_positions,
                        }
                    } else {
                        MoveResult::IllegalMove {}
                    }
                }
                WordCheckResult::Legal { word } => MoveResult::WordComplete {
                    word,
                    coordinates: new_chosen_positions,
                },
                WordCheckResult::Illegal { word: _ } => MoveResult::WordContinued {
                    word,
                    coordinates: new_chosen_positions,
                },
            };
            return final_result;
        }

        MoveResult::IllegalMove {}
    }

    fn get_word_text(&self, coordinates: &[Coordinate]) -> String {
        let word = coordinates
            .iter()
            .map(|c| {
                let letter = &self.board.get_letter_at_coordinate(c);

                letter.word_text()
            })
            .join("");
        word
    }

    pub fn after_move_result(self, move_result: &MoveResult) -> Self {
        match move_result {
            MoveResult::WordComplete {
                word: _,
                coordinates,
            } => Self {
                chosen_positions: coordinates.clone(),
                ..self
            },
            MoveResult::WordContinued {
                word: _,
                coordinates,
            } => Self {
                chosen_positions: coordinates.clone(),
                ..self
            },
            MoveResult::WordAbandoned => Self {
                chosen_positions: Default::default(),
                ..self
            },
            MoveResult::MoveRetraced {
                word: _,
                coordinates,
            } => Self {
                chosen_positions: coordinates.clone(),
                ..self
            },
            MoveResult::IllegalMove => self,
        }
    }
}
