use crate::core::prelude::*;
use crate::state::prelude::*;
use serde::*;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct FullState {
    pub board: Rc<Board>,
    pub found_words: Rc<FoundWordsState>,
    pub solve_settings: SolveSettings,
}

impl FullState {}
