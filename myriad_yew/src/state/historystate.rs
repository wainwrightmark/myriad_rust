use myriad::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use serde::*;
use std::{collections::BTreeMap, rc::Rc};
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct HistoryState {
    pub games: Vec<(Game, BTreeMap<i32, FoundWord<GRID_COLUMNS, GRID_ROWS>>)>,
}

pub struct SaveGameMessage {
    pub game: Game,
    pub found_words: BTreeMap<i32, FoundWord<GRID_COLUMNS, GRID_ROWS>>,
}

impl Reducer<HistoryState> for SaveGameMessage {
    fn apply(self, state: std::rc::Rc<HistoryState>) -> std::rc::Rc<HistoryState> {
        if self.found_words.is_empty() {
            return state; //no need to save
        }

        let mut new_state = state.as_ref().clone();
        if let Some((index, _)) = state
            .games
            .iter()
            .find_position(|x| x.0.board == self.game.board)
        {
            new_state.games[index].1 = self.found_words.clone();
        } else {
            new_state
                .games
                .push((self.game.clone(), self.found_words.clone()));
        }

        Rc::new(new_state)
    }
}
