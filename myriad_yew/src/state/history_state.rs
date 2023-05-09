use crate::state::prelude::*;
use itertools::Itertools;
use serde::*;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)] // can also be "session"
pub struct HistoryState {
    pub games: Vec<FullGameState>,
}


impl HistoryState{
    pub fn all_games_including_current<'a, 'b>  (&'a self, current: &'b FullGameState)-> impl Iterator<Item = &'a FullGameState> where 'b : 'a{
        self.games.iter().filter(|x|x.game.board != current.game.board) .chain(std::iter::once(current)).rev()

    }
}

pub struct SaveGameMessage(pub Rc<FullGameState>);

impl Reducer<HistoryState> for SaveGameMessage {
    fn apply(self, state: std::rc::Rc<HistoryState>) -> std::rc::Rc<HistoryState> {
        if self.0.found_words.words.is_empty() {
            return state; //no need to save
        }

        let mut new_state = state.as_ref().clone();
        if let Some((index, _)) = state
            .games
            .iter()
            .find_position(|x| x.game.board == self.0.game.board)
        {
            new_state.games[index].found_words = self.0.found_words.clone();
        } else {
            new_state.games.push(self.0.as_ref().clone());
        }

        Rc::new(new_state)
    }
}
