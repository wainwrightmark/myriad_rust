use std::collections::BTreeMap;
use crate::core::solver::*;

#[derive(PartialEq,Clone)]
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

        return FoundWordsState { words: new_map };
    }

    pub fn has_word(&self, word: &FoundWord)-> bool{
        self.words.contains_key(&word.result)
    }
}
