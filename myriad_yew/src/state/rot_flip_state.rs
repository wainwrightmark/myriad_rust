use crate::state::prelude::*;
use myriad::prelude::{QuarterTurns, Tile};
use serde::*;
use yewdux::prelude::*;

pub struct RotFlipMsg {
    pub rotate: QuarterTurns,
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
    pub rotate: QuarterTurns,
    pub flip: bool,
}

impl RotFlipState {
    pub fn get_location(
        &self,
        point: &Tile<GRID_COLUMNS, GRID_ROWS>,
        square_size: f32,
    ) -> (f32, f32) {
        let flipped = rotate_and_flip(point, self.rotate, self.flip);

        let cx = (flipped.col() as f32 + 0.5) * square_size;
        let cy = (flipped.row() as f32 + 0.5) * square_size;

        (cx, cy)
    }

    pub fn total_letters(&self) -> usize {
        (GRID_COLUMNS * GRID_ROWS) as usize
    }

    pub fn new_game(&mut self) {
        self.rotate = self.rotate + QuarterTurns::One;
        self.flip = !self.flip;
    }
}

pub fn rotate_and_flip<const L: u8>(
    point: &Tile<L, L>,
    rotate: QuarterTurns,
    flip: bool,
) -> Tile<L, L> {
    let rotated = point.rotate(rotate);

    if flip {
        rotated.flip(myriad::prelude::FlipAxes::Vertical)
    } else {
        rotated
    }
}
