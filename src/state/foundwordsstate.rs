use crate::core::prelude::*;
use crate::state::GOALSIZE;
use num::{iter::Range, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct FoundWordsState {
    pub words: BTreeMap<i32, FoundWord>,
    pub most_recent: Option<i32>,
    pub locked_tab_index: Option<usize>,
}

impl FoundWordsState {
    pub fn with_word(&self, word: FoundWord) -> Self {
        let mut new_map = self.words.clone();

        let i = word.result;
        new_map.insert(i, word);

        FoundWordsState {
            words: new_map,
            most_recent: Some(i),
            locked_tab_index: self.locked_tab_index,
        }
    }

    pub fn has_word(&self, word: &FoundWord) -> bool {
        self.words.contains_key(&word.result)
    }

    pub fn selected_index(&self) -> usize {
        if let Some(i) = self.locked_tab_index {
            i
        } else if let Some(r) = self.most_recent {
            ((r - 1) / 20).to_usize().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn is_goal_complete(&self, index: i32) -> bool {
        self.has_all_words(&mut num::iter::range(
            (index * GOALSIZE) + 1,
            ((index + 1) * GOALSIZE) + 1,
        ))
    }

    pub fn has_all_words(&self, range: &mut Range<i32>) -> bool {
        if self.words.len() < range.size_hint().0 {
            return false;
        }

        range.all(|x| self.words.contains_key(&x))
    }
}
