use crate::core::prelude::*;
use crate::state::prelude::*;
use num::{iter::Range, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct FoundWordsState {
    pub words: BTreeMap<i32, FoundWord>,
    pub most_recent: Option<i32>,
}

impl FoundWordsState {
    pub fn with_word(&self, word: FoundWord) -> Self {
        let mut new_map = self.words.clone();

        let i = word.result;
        new_map.insert(i, word);

        FoundWordsState {
            words: new_map,
            most_recent: Some(i),
        }
    }

    pub fn has_word(&self, word: &FoundWord) -> bool {
        self.words.contains_key(&word.result)
    }

    pub fn is_goal_complete(&self, index: usize) -> bool {
        let u = index.to_i32().unwrap();

        self.has_all_words(&mut num::iter::range(
            (u * GOALSIZE) + 1,
            ((u + 1) * GOALSIZE) + 1,
        ))
    }

    pub fn has_all_words(&self, range: &mut Range<i32>) -> bool {
        if self.words.len() < range.size_hint().0 {
            return false;
        }

        range.all(|x| self.words.contains_key(&x))
    }
}
