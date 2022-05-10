use num::ToPrimitive;
use serde::*;

use crate::core::prelude::Coordinate;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RotFlipState {
    pub rotate: i8,
    pub flip: bool,
    pub max_coordinate: Coordinate,
}

impl Default for RotFlipState {
    fn default() -> Self {
        Self {
            max_coordinate: Coordinate { row: 2, column: 2 },
            rotate: Default::default(),
            flip: Default::default(),
        }
    }
}

impl RotFlipState {
    pub fn get_location(&self, coordinate: &Coordinate, square_size: f64) -> (f64, f64) {
        let rotated = coordinate.rotate_and_flip(self.max_coordinate, self.rotate, self.flip);

        let cx = (rotated.column as f64 + 0.5) * square_size;
        let cy = (rotated.row as f64 + 0.5) * square_size;

        (cx, cy)
    }

    pub fn total_letters(&self) -> usize {
        self.columns() * self.rows()
    }

    pub fn columns(&self) -> usize {
        (self.max_coordinate.column + 1).to_usize().unwrap()
    }

    pub fn rows(&self) -> usize {
        (self.max_coordinate.row + 1).to_usize().unwrap()
    }
}
