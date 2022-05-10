automod::dir!("src/web");

pub const SQUARE_SIZE: f64 = 120.0;
pub const SVG_WIDTH: f64 = SQUARE_SIZE * 3.0;
pub const SVG_HEIGHT: f64 = BOARD_HEIGHT + TAB_HEADER_HEIGHT + FOUND_WORD_HEIGHT * 2.0 + FOUND_WORD_MARGIN * 3.0;

pub const BOARD_HEIGHT : f64 = SQUARE_SIZE * 3.0;


pub const TAB_HEADER_WIDTH : f64 = 60.0;
pub const TAB_HEADER_HEIGHT : f64 = 60.0;

pub const FOUND_WORD_WIDTH : f64 = 30.0;
pub const FOUND_WORD_HEIGHT : f64 = 30.0;
pub const FOUND_WORD_MARGIN : f64 = 5.0;
pub const FOUND_WORD_PADDING : f64 = (SVG_WIDTH - (FOUND_WORD_WIDTH * 10.0 + FOUND_WORD_MARGIN * 9.0)) / 2.0;


pub mod prelude {

    pub use crate::web::board::*;
    pub use crate::web::confetti::*;
    pub use crate::web::foundwords::*;
    pub use crate::web::newgamebutton::*;
    pub use crate::web::recentwords::*;
    pub use crate::web::rope::*;
}
