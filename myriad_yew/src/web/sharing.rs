use wasm_bindgen_futures::spawn_local;

use web_sys::window;
use web_sys::ShareData;
use yewdux::prelude::Dispatch;

use crate::state::full_game_state::FullGameState;

pub fn share() {
    spawn_local(async {
        share_async().await;
    });
}

pub async fn share_async() {
    let Some(window) =  window() else {return;};
    let navigator = window.navigator();

    let state = Dispatch::<FullGameState>::new().get();
    let game_text = state.game.board.to_single_string();
    let url = format!("https://myriad-game.com/?game={game_text}");

    let mut share_data: ShareData = ShareData::new();

    share_data.text("Myriad - The game where you find all the numbers.");
    share_data.title("Myriad");
    share_data.url(url.as_str());
    let promise = navigator.share_with_data(&share_data);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;

    match result {
        Ok(js_value) => {
            log::info!("Share Succeeded: {js_value:?}");
        }
        Err(js_value) => {
            log::info!("Share Failed: {js_value:?}");
        }
    }
}
