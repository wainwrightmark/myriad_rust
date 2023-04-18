use capacitor_bindings::share::ShareOptions;
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

    let state = Dispatch::<FullGameState>::new().get();
    let game_text = state.game.board.canonical_string();
    let url = format!("https://myriad-game.com/?game={game_text}");

    let result = capacitor_bindings::share::Share::share(ShareOptions{
        title: Some("Myriad".to_string()),
        text: Some("Find every number from one to one hundred.".to_string()),
        url: Some(url),
        dialog_title: Some("Myriad Share".to_string()),
        files: None,
    }).await;
}
