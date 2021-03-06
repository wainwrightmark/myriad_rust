pub mod chosenpositionsstate;
pub mod circletype;
pub mod foundwordsstate;
pub mod fullstate;
pub mod msg;
pub mod recentwordstate;
pub mod rotflipstate;
pub mod selectedtabstate;
pub mod dialogstate;
pub mod historystate;

pub mod prelude {

    pub use crate::state::chosenpositionsstate::*;
    pub use crate::state::circletype::*;
    pub use crate::state::foundwordsstate::*;
    pub use crate::state::fullstate::*;
    pub use crate::state::msg::*;
    pub use crate::state::recentwordstate::*;
    pub use crate::state::rotflipstate::*;
    pub use crate::state::selectedtabstate::*;
    pub use crate::state::dialogstate::*;
    pub use crate::state::historystate::*;

    pub const GOALSIZE: i32 = 20;

    pub const GRID_COLUMNS: usize = 3;
    pub const GRID_ROWS: usize = 3;
}
