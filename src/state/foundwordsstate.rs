use crate::core::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct FoundWordsState {
    pub words: BTreeMap<i32, FoundWord>,
}

impl Default for FoundWordsState {
    fn default() -> Self {
        Self {
            words: Default::default(),
        }
    }
}

impl FoundWordsState {
    pub fn with_word(&self, word: FoundWord) -> Self {
        let mut new_map = self.words.clone();

        new_map.insert(word.result, word);

        FoundWordsState { words: new_map }
    }

    pub fn has_word(&self, word: &FoundWord) -> bool {
        self.words.contains_key(&word.result)
    }
}
