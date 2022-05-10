use crate::state::fullstate::*;
use crate::state::msg::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(NewGameButton)]
pub fn new_game_button() -> Html {
    let dispatch = Dispatch::<FullState>::new();

    let onclick = dispatch.apply_callback(|_| Msg::NewGame);

    html! {
        <div>
            <button {onclick }>{"New Game"} </button>
        </div>
    }
}
