#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod board;
mod coordinate;
mod creator;
mod game_mode;
mod move_result;
mod rune;
mod solver;

pub mod parser;

pub mod prelude {

    pub use crate::board::*;
    pub use crate::coordinate::*;
    pub use crate::creator::*;
    pub use crate::game_mode::*;
    pub use crate::move_result::*;
    pub use crate::rune::*;
    pub use crate::solver::*;
}
