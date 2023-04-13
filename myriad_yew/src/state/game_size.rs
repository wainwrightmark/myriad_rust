use myriad::prelude::Center;
use yewdux::store::{Reducer, Store};

use super::prelude::*;
/// The size of the game area
#[derive(Copy, Clone, PartialEq, Debug, Store)]
pub struct GameSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SetSizeMessage {
    pub width: u32,
    pub height: u32,
}

impl Reducer<GameSize> for SetSizeMessage {
    fn apply(self, mut state: std::rc::Rc<GameSize>) -> std::rc::Rc<GameSize> {
        if self == Default::default() {
            return state;
        }
        let w = self.width as f32;
        let h = self.height as f32;

        if w == state.width && h == state.height {
            return state;
        }

        let s = std::rc::Rc::make_mut(&mut state);
        s.width = w;
        s.height = h;

        state
    }
}

impl Default for GameSize {
    fn default() -> Self {
        Self {
            width: 400.,
            height: 400.,
        }
    }
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
        self.width.min(self.height * 8. / 13.0)
    }

    /// The length of one of the board squares
    pub fn square_length(&self) -> f32 {
        self.board_length() / 3.
    }

    /// The length of one of the board squares
    pub fn square_radius(&self) -> f32 {
        self.board_length() / 6.
    }

    fn circle_diameter(&self) -> f32 {
        self.square_length() * crate::web::prelude::CIRCLE_RATIO
    }

    pub fn tab_header_diameter(&self) -> f32 {
        self.board_length() / 8.0
    }

    pub fn style_string(&self) -> String {
        let circle_diameter = self.circle_diameter();
        let circle_radius = circle_diameter * 0.5;

        let tab_header_diameter = self.tab_header_diameter();
        let tab_header_font_size = tab_header_diameter / 1.5;

        format!("--circle-diameter: {circle_diameter}px; --circle-radius: {circle_radius}px; --tab-header-diameter: {tab_header_diameter}px; --tab-header-font-size: {tab_header_font_size}px;")
    }

    pub fn get_info_bar_position(&self) -> (f32, f32) {
        let y = self.board_length();

        let x = 0.0;

        (x, y)
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
            + self.tab_header_diameter()
            + TAB_HEADER_TOP_MARGIN
            + FOUND_WORD_MARGIN
            + INFO_BAR_HEIGHT
            + (FOUND_WORD_HEIGHT + FOUND_WORD_MARGIN) * row_number as f32;

        let row_position = ((number - 1) % GOALSIZE) % 10;

        let found_word_padding =
            (self.width - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;

        let tab_x =
            found_word_padding + row_position as f32 * (FOUND_WORD_MARGIN + FOUND_WORD_WIDTH);

        let index = (number - 1) / GOALSIZE;
        let mut index_offset = (index - selected_index as i32) as f32;
        if clamp {
            index_offset = index_offset.min(1.0).max(-1.0);
        }

        let offset_x = index_offset * self.board_length();

        let x = tab_x + offset_x;
        (x, y)
    }

    pub fn get_tab_header_padding(&self) -> f32 {
        (self.width - ((self.tab_header_diameter() + 3.0) * 6.0 + TAB_HEADER_MARGIN * 5.0)) / 2.0
    }
}

pub const TAB_HEADER_TOP_MARGIN: f32 = 40.0;

pub const INFO_BAR_HEIGHT: f32 = 60.0;

pub const TAB_HEADER_MARGIN: f32 = 6.0;
//pub const TAB_HEADER_WIDTH: f32 = 50.0;
//pub const TAB_HEADER_HEIGHT: f32 = 50.0;
pub const FOUND_WORD_WIDTH: f32 = 30.0;
pub const FOUND_WORD_HEIGHT: f32 = 30.0;
pub const FOUND_WORD_MARGIN: f32 = 5.0;
pub const FOUND_WORD_TOP_PADDING: f32 = 10.0;
