use crate::state::prelude::*;
use myriad::prelude::PointAbsolute8;
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
        point: &PointAbsolute8<GRID_COLUMNS, GRID_ROWS>,
        square_size: f32,
    ) -> (f32, f32) {
        let flipped = rotate_and_flip(point, self.rotate, self.flip);

        let cx = (flipped.x() as f32 + 0.5) * square_size;
        let cy = (flipped.y() as f32 + 0.5) * square_size;

        (cx, cy)
    }

    pub fn total_letters(&self) -> usize {
        (GRID_COLUMNS * GRID_ROWS) as usize
    }

    pub fn new_game(&mut self) {
        self.rotate = (self.rotate + 1) % 4;
        self.flip = !self.flip;
    }
}

pub fn rotate_and_flip<const L: u8>(
    point: &PointAbsolute8<L, L>,
    mut rotate: i8,
    flip: bool,
) -> PointAbsolute8<L, L> {
    while rotate < 0 {
        rotate += 4
    }

    let rotated = point.rotate(rotate as u8); // .rotate_and_flip(self.rotate, self.flip);
    let flipped = if flip {
        rotated.flip_vertical()
    } else {
        rotated
    };
    flipped
}
