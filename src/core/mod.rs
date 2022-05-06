#[allow(dead_code)]
pub mod letter;

#[allow(dead_code)]
pub mod coordinate;

#[allow(dead_code)]
pub mod solver;

#[allow(dead_code)]
pub mod board;

#[allow(dead_code)]
pub mod creator;

#[allow(dead_code)]
pub mod move_result;

pub mod prelude{

    pub use crate::core::board::*;
    pub use crate::core::coordinate::*;
    pub use crate::core::creator::*;
    pub use crate::core::letter::*;
    pub use crate::core::move_result::*;
    pub use crate::core::solver::*;
}