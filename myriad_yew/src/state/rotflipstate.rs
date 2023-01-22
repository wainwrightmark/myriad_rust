use myriad::prelude::Coordinate;
use crate::state::prelude::*;
use serde::*;
use yewdux::prelude::*;

pub struct RotFlipMsg {
    pub rotate: i8,
    pub flip: bool,
}

impl Reducer<RotFlipState> for RotFlipMsg {
    fn apply(self, state: std::rc::Rc<RotFlipState>) -> std::rc::Rc<RotFlipState> {
        RotFlipState {
            rotate: state.rotate + self.rotate,
            flip: state.flip ^ self.flip,
        }
        .into()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Store, Debug, Default)]
#[store(storage = "local")] // can also be "session"
pub struct RotFlipState {
    pub rotate: i8,
    pub flip: bool,
}

impl RotFlipState {
    pub fn get_location(
        &self,
        coordinate: &Coordinate<GRID_COLUMNS, GRID_ROWS>,
        square_size: f64,
    ) -> (f64, f64) {
        let rotated = coordinate.rotate_and_flip(self.rotate, self.flip);

        let cx = (rotated.get_column() as f64 + 0.5) * square_size;
        let cy = (rotated.get_row() as f64 + 0.5) * square_size;

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