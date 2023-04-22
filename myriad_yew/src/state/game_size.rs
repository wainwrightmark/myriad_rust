use myriad::prelude::Center;
use yewdux::store::{Reducer, Store};

use super::prelude::*;
/// The size of the game area
#[derive(Copy, Clone, PartialEq, Debug, Store)]
pub struct GameSize {
    pub board_length: f32,

    pub outer_length: f32,
    // pub width: f32,
    // pub height: f32,
    pub orientation: Orientation,
}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Default)]
pub enum Orientation {
    #[default]
    /// Taller than it is wide
    Vertical,
    /// Wider than it is tall
    Horizontal,
}

#[derive(Debug, Default, PartialEq)]
pub struct SetSizeMessage {
    pub width: f32,
    pub height: f32,
}

impl Reducer<GameSize> for SetSizeMessage {
    fn apply(self, mut state: std::rc::Rc<GameSize>) -> std::rc::Rc<GameSize> {
        if self == Default::default() {
            return state;
        }

        let s = std::rc::Rc::make_mut(&mut state);
        *s = GameSize::from_width_and_height(self.width as f32, self.height as f32);

        state
    }
}

impl Default for GameSize {
    fn default() -> Self {
        Self {
            board_length: 400.,
            outer_length: 650.,

            orientation: Orientation::default(),
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
    pub fn from_width_and_height(width: f32, height: f32) -> Self {
        let orientation = if width <= height {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };

        let board_length = match orientation {
            Orientation::Vertical => width.min(height * 8. / 13.0),
            Orientation::Horizontal => height.min(width * 8. / 13.),
        }
        .min(400.);

        let outer_length = match orientation{
            Orientation::Vertical=> width.min(board_length * 1.5),
            Orientation::Horizontal => height.min(board_length * 1.2)
        };


        Self {
            orientation,
            board_length,
            outer_length
        }
    }

    /// The length of one of the board squares
    pub fn square_length(&self) -> f32 {
        self.board_length / 3.
    }

    /// The length of one of the board squares
    pub fn square_radius(&self) -> f32 {
        self.board_length / 6.
    }

    fn circle_diameter(&self) -> f32 {
        self.square_length() * crate::web::prelude::CIRCLE_RATIO
    }

    pub fn tab_header_diameter(&self) -> f32 {
        self.board_length / 8.0
    }

    pub fn outer_container_style(&self) -> String {

        let board_other_length = self.board_length * 13./ 8.;
        let board_length = self.outer_length;

        match self.orientation {
            Orientation::Vertical => format!("width: {board_length}px;") ,
            Orientation::Horizontal => format!("height: {board_length}px; width: {board_other_length}px;"),
        }
    }

    pub fn container_style(&self) -> String {
        let circle_diameter = self.circle_diameter();
        let circle_radius = circle_diameter * 0.5;

        let tab_header_diameter = self.tab_header_diameter();
        let tab_header_font_size = tab_header_diameter / 1.5;

        let board_other_length = self.board_length * 13./ 8.;
        let board_length = self.board_length;

        match self.orientation {
            Orientation::Vertical => format!("width: {board_length}px; height: {board_other_length}px; --circle-diameter: {circle_diameter}px; --circle-radius: {circle_radius}px; --tab-header-diameter: {tab_header_diameter}px; --tab-header-font-size: {tab_header_font_size}px;"),
            Orientation::Horizontal => format!("height: {board_length}px; width: {board_other_length}px; --circle-diameter: {circle_diameter}px; --circle-radius: {circle_radius}px; --tab-header-diameter: {tab_header_diameter}px; --tab-header-font-size: {tab_header_font_size}px;"),
        }
    }

    pub fn get_info_bar_position(&self) -> (f32, f32) {
        match self.orientation {
            Orientation::Vertical => {
                let x = 0.0;
                let y = self.board_length;

                (x, y)
            }
            Orientation::Horizontal => {
                let x = self.board_length;
                let y = 0.0;
                (x, y)
            }
        }
    }
    pub fn get_info_bar_size(&self) -> (f32, f32) {
        let width: f32;
        let height: f32;
        match self.orientation {
            Orientation::Vertical => {
                width = self.board_length;
                height = INFO_BAR_HEIGHT;
            }
            Orientation::Horizontal => {
                width = INFO_BAR_HEIGHT;
                height = self.board_length;
            }
        }

        (width, height)
    }

    pub fn get_found_word_position(
        &self,
        number: i32,
        selected_index: usize,
        clamp: bool,
    ) -> (f32, f32) {

        let number  = if number <= 100{ number + 20} else { number % 100};
        let row_number = ((number - 1) % GOALSIZE) / 10;
        let row_position = ((number - 1) % GOALSIZE) % 10;

        match self.orientation {
            Orientation::Vertical => {
                let y = self.board_length
                    + FOUND_WORD_TOP_PADDING
                    + self.tab_header_diameter()
                    + TAB_HEADER_TOP_MARGIN
                    + FOUND_WORD_MARGIN
                    + INFO_BAR_HEIGHT
                    + (FOUND_WORD_HEIGHT + FOUND_WORD_MARGIN) * row_number as f32;

                let found_word_padding =
                    (self.board_length - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;

                let tab_x = found_word_padding
                    + row_position as f32 * (FOUND_WORD_MARGIN + FOUND_WORD_WIDTH);

                let index = (number - 1) / GOALSIZE;
                let mut index_offset = (index - selected_index as i32) as f32;
                if clamp {
                    index_offset = index_offset.min(1.0).max(-1.0);
                }

                let offset_x = index_offset * self.board_length;

                let x = tab_x + offset_x;
                (x, y)
            }
            Orientation::Horizontal => {
                let x = self.board_length
                    + FOUND_WORD_TOP_PADDING
                    + self.tab_header_diameter()
                    + TAB_HEADER_TOP_MARGIN
                    + FOUND_WORD_MARGIN
                    + INFO_BAR_HEIGHT
                    + (FOUND_WORD_HEIGHT + FOUND_WORD_MARGIN) * row_number as f32;

                let found_word_padding =
                    (self.board_length - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;

                let tab_y = found_word_padding
                    + row_position as f32 * (FOUND_WORD_MARGIN + FOUND_WORD_HEIGHT);

                let index = (number - 1) / GOALSIZE;
                let mut index_offset = (index - selected_index as i32) as f32;
                if clamp {
                    index_offset = index_offset.min(1.0).max(-1.0);
                }

                let offset_y = index_offset * self.board_length;

                let y = tab_y + offset_y;
                (x, y)
            }
        }
    }

    pub fn get_tab_header_position(&self, index: usize) -> (f32, f32) {
        match self.orientation {
            Orientation::Vertical => {
                let tab_header_padding = (self.board_length
                    - ((self.tab_header_diameter() + 3.0) * 6.0 + TAB_HEADER_MARGIN * 5.0))
                    / 2.0;

                let x = tab_header_padding
                    + (index as f32 * (self.tab_header_diameter() + TAB_HEADER_MARGIN));
                let y = (self.square_length() * 3.0) + TAB_HEADER_TOP_MARGIN + INFO_BAR_HEIGHT;
                (x, y)
            }
            Orientation::Horizontal => {
                let tab_header_padding = (self.board_length
                    - ((self.tab_header_diameter() + 3.0) * 6.0 + TAB_HEADER_MARGIN * 5.0))
                    / 2.0;

                let x = self.board_length + INFO_BAR_HEIGHT + TAB_HEADER_TOP_MARGIN;
                let y = tab_header_padding
                    + (index as f32 * (self.tab_header_diameter() + TAB_HEADER_MARGIN));

                (x, y)
            }
        }
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
