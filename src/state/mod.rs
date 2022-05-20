pub mod chosenpositionsstate;
pub mod circletype;
pub mod drag;
pub mod foundwordsstate;
pub mod fullstate;
pub mod msg;
pub mod recentwordstate;
pub mod rotflipstate;
pub mod selectedtabstate;

pub mod prelude {

    pub use crate::state::chosenpositionsstate::*;
    pub use crate::state::circletype::*;
    pub use crate::state::drag::*;
    pub use crate::state::foundwordsstate::*;
    pub use crate::state::fullstate::*;
    pub use crate::state::msg::*;
    pub use crate::state::recentwordstate::*;
    pub use crate::state::rotflipstate::*;
    pub use crate::state::selectedtabstate::*;

    pub const GOALSIZE: i32 = 20;
}
