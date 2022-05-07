use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/confetti.js")]
extern "C" {
    pub fn make_confetti(emoji: String, number: i32);
}

