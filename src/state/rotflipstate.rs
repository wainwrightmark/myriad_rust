use crate::{core::prelude::Coordinate, state::prelude::*};
use serde::*;
use yewdux::prelude::*;

pub struct RotFlipMsg {
    rotate: i8,
    flip: bool,
}

impl Reducer<RotFlipState> for RotFlipMsg {
    fn apply(&self, state: std::rc::Rc<RotFlipState>) -> std::rc::Rc<RotFlipState> {
        RotFlipState {
            rotate: state.rotate + self.rotate,
            flip: state.flip ^ self.flip,
        }
        .into()
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Store, Debug, Default)]
pub struct RotFlipState {
    pub rotate: i8,
    pub flip: bool,
}

impl RotFlipState {
    pub fn get_location(&self, coordinate: &Coordinate, square_size: f64) -> (f64, f64) {
        let rotated = coordinate.rotate_and_flip::<GRID_COLUMNS, GRID_ROWS>(self.rotate, self.flip);

        let cx = (rotated.column as f64 + 0.5) * square_size;
        let cy = (rotated.row as f64 + 0.5) * square_size;

        (cx, cy)
    }

    pub fn total_letters(&self) -> usize {
        GRID_COLUMNS * GRID_ROWS
    }

    pub fn new_game(&mut self) {
        self.rotate = (self.rotate + 1) % 4;
        self.flip = !self.flip;
    }
}
