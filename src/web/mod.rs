automod::dir!("src/web");

pub const SQUARE_SIZE: f64 = 120.0;
pub const SVG_WIDTH: f64 = 360.0;
pub const SVG_HEIGHT: f64 = 360.0;


pub mod prelude {

    pub use crate::web::board::*;
    pub use crate::web::confetti::*;
    pub use crate::web::foundwords::*;
    pub use crate::web::newgamebutton::*;
    pub use crate::web::recentwords::*;
    pub use crate::web::rope::*;
}
