use myriad::prelude::Center;
use num::ToPrimitive;
use yew::Properties;
use super::prelude::GOALSIZE;
/// The size of the game area
#[derive(Copy, Clone, PartialEq, Debug, Properties)]
pub struct GameSize {
    pub width: f32,
    pub height: f32,
}

pub trait CenterStyle {
    fn get_style(&self) -> String;
}

impl CenterStyle for Center {
    fn get_style(&self) -> String {
        format!("left: {}px; top: {}px", self.x, self.y)
    }
}



impl GameSize {
    pub fn board_length(&self) -> f32 {
        self.width.max(self.height / 1.5)
    }

    /// The length of one of the board squares
    pub fn square_length(&self) -> f32 {
        self.board_length() / 3.
    }

    /// The length of one of the board squares
    pub fn square_radius(&self) -> f32 {
        self.board_length() / 6.
    }

    pub fn get_found_word_position(
        &self,
        number: i32,
        selected_index: usize,
        clamp: bool,
    ) -> (f32, f32) {
        let row_number = ((number - 1) % GOALSIZE) / 10;
        let y = self.board_length()
            + FOUND_WORD_TOP_PADDING
            + TAB_HEADER_HEIGHT
            + TAB_HEADER_TOP_MARGIN
            + FOUND_WORD_MARGIN
            + (FOUND_WORD_HEIGHT + FOUND_WORD_MARGIN) * row_number.to_f32().unwrap();

        let row_position = ((number - 1) % GOALSIZE) % 10;

        let found_word_padding =
            (self.width - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;

        let tab_x = found_word_padding
            + row_position.to_f32().unwrap() * (FOUND_WORD_MARGIN + FOUND_WORD_WIDTH);

        let index = (number - 1) / GOALSIZE;
        let mut index_offset = (index - selected_index.to_i32().unwrap()).to_f32().unwrap();
        if clamp {
            index_offset = index_offset.min(1.0).max(-1.0);
        }

        let offset_x = index_offset * self.board_length();

        let x = tab_x + offset_x;
        (x, y)
    }
}

pub const TAB_HEADER_TOP_MARGIN: f32 = 40.0;

pub const TAB_HEADER_MARGIN: f32 = 6.0;
pub const TAB_HEADER_WIDTH: f32 = 50.0;
pub const TAB_HEADER_HEIGHT: f32 = 50.0;
pub const FOUND_WORD_WIDTH: f32 = 30.0;
pub const FOUND_WORD_HEIGHT: f32 = 30.0;
pub const FOUND_WORD_MARGIN: f32 = 5.0;
pub const FOUND_WORD_TOP_PADDING: f32 = 10.0;

pub const TAB_HEADER_PADDING: f32 =
        (400.0 - ((TAB_HEADER_WIDTH + 3.0) * 6.0 + TAB_HEADER_MARGIN * 5.0)) / 2.0;