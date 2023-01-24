mod app;
mod circle;
mod confetti;
mod controlbuttons;
mod crosshair;
mod dialog;
mod foundwords;
mod icons;
mod recentwords;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::circle::*;
    pub use crate::web::confetti::*;
    pub use crate::web::controlbuttons::*;
    pub use crate::web::crosshair::*;
    pub use crate::web::dialog::*;
    pub use crate::web::foundwords::*;
    pub use crate::web::icons::*;
    pub use crate::web::recentwords::*;

    pub fn format_number(num: i32) -> String {
        if num == 100 {
            "💯".to_string()
        } else if num < 10 {
            format!("{num:0>1}")
        } else {
            format!("{num:0>2}")
        }
    }

    pub const SQUARE_SIZE: f32 = 120.0;

    pub const CIRCLE_RATIO: f32 = 0.8;

    // pub const SQUARE_MIDPOINT: f32 = SQUARE_SIZE / 2.0;
    pub const SVG_WIDTH: f32 = SQUARE_SIZE * 3.0;
    pub const SVG_HEIGHT: f32 = BOARD_HEIGHT
        + TAB_HEADER_TOP_MARGIN
        + TAB_HEADER_HEIGHT
        + FOUND_WORD_HEIGHT * 2.0
        + FOUND_WORD_MARGIN * 3.0;

    pub const BOARD_HEIGHT: f32 = SQUARE_SIZE * 3.0;

    pub const TAB_HEADER_TOP_MARGIN: f32 = 60.0;

    pub const TAB_HEADER_MARGIN: f32 = 6.0;
    pub const TAB_HEADER_WIDTH: f32 = 50.0;
    pub const TAB_HEADER_HEIGHT: f32 = 50.0;
    pub const TAB_HEADER_PADDING: f32 =
        (SVG_WIDTH - (TAB_HEADER_WIDTH * 6.0 + TAB_HEADER_MARGIN * 5.0)) / 2.0;

    pub const FOUND_WORD_WIDTH: f32 = 30.0;
    pub const FOUND_WORD_HEIGHT: f32 = 30.0;
    pub const FOUND_WORD_MARGIN: f32 = 5.0;
    pub const FOUND_WORD_PADDING: f32 =
        (SVG_WIDTH - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;
}
