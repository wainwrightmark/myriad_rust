automod::dir!("src/core");

pub mod parser;

pub mod prelude {

    pub use crate::core::board::*;
    pub use crate::core::coordinate::*;
    pub use crate::core::creator::*;
    pub use crate::core::letter::*;
    pub use crate::core::move_result::*;
    pub use crate::core::solver::*;
}
