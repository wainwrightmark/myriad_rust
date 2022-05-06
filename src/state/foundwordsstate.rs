use crate::core::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct FoundWordsState {
    pub words: BTreeMap<i32, FoundWord>,
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
