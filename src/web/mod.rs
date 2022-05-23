mod app;
mod circle;
mod confetti;
mod crosshair;
mod foundwords;
mod newgamebutton;
mod recentwords;
mod rope;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::circle::*;
    pub use crate::web::confetti::*;
    pub use crate::web::crosshair::*;
    pub use crate::web::foundwords::*;
    pub use crate::web::newgamebutton::*;
    pub use crate::web::recentwords::*;
    pub use crate::web::rope::*;

    pub fn format_number(num: i32) -> String {
        if num == 100 {
            "💯".to_string()
        } else if num < 10 {
            format!("{:0>1}", num)
        } else {
            format!("{:0>2}", num)
        }
    }

   

    pub const SQUARE_SIZE: f64 = 120.0;
    pub const SQUARE_MIDPOINT: f64 = SQUARE_SIZE / 2.0;
    pub const SVG_WIDTH: f64 = SQUARE_SIZE * 3.0;
    pub const SVG_HEIGHT: f64 = BOARD_HEIGHT
        + TAB_HEADER_TOP_MARGIN
        + TAB_HEADER_HEIGHT
        + FOUND_WORD_HEIGHT * 2.0
        + FOUND_WORD_MARGIN * 3.0;

    pub const BOARD_HEIGHT: f64 = SQUARE_SIZE * 3.0;

    pub const TAB_HEADER_TOP_MARGIN: f64 = 60.0;

    pub const TAB_HEADER_MARGIN: f64 = 6.0;
    pub const TAB_HEADER_WIDTH: f64 = 50.0;
    pub const TAB_HEADER_HEIGHT: f64 = 50.0;
    pub const TAB_HEADER_PADDING: f64 =
        (SVG_WIDTH - (TAB_HEADER_WIDTH * 6.0 + TAB_HEADER_MARGIN * 5.0)) / 2.0;

    pub const FOUND_WORD_WIDTH: f64 = 30.0;
    pub const FOUND_WORD_HEIGHT: f64 = 30.0;
    pub const FOUND_WORD_MARGIN: f64 = 5.0;
    pub const FOUND_WORD_PADDING: f64 =
        (SVG_WIDTH - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;
}
