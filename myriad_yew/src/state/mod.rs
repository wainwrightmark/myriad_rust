pub mod chosen_positions_state;
pub mod circle_type;
pub mod dialog_state;
pub mod failed_logs_state;
pub mod found_words_state;
pub mod full_game_state;
pub mod game_size;
pub mod history_state;
pub mod info_bar_state;
pub mod logging;
pub mod msg;
pub mod preferences_state;
pub mod recent_word_state;
pub mod rot_flip_state;
pub mod selected_tab_state;
pub mod user_state;
pub mod util;

pub mod prelude {

    pub use crate::state::chosen_positions_state::*;
    pub use crate::state::circle_type::*;
    pub use crate::state::dialog_state::*;
    pub use crate::state::found_words_state::*;
    pub use crate::state::full_game_state::*;
    pub use crate::state::game_size::*;
    pub use crate::state::history_state::*;
    pub use crate::state::msg::*;
    pub use crate::state::recent_word_state::*;
    pub use crate::state::rot_flip_state::*;
    pub use crate::state::selected_tab_state::*;
    pub use crate::state::util::*;

    pub use crate::state::failed_logs_state::*;
    pub use crate::state::logging::*;
    pub use crate::state::logging::*;

    pub const GOALSIZE: i32 = 20;

    pub const GRID_COLUMNS: u8 = 3;
    pub const GRID_ROWS: u8 = 3;
    pub const GRID_SIZE: usize = 9;
}
