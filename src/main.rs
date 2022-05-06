use yew::prelude::*;


use crate::web::board::*;
use crate::web::recentwords::*;
use crate::web::foundwords::*;
use crate::web::newgamebutton::*;
pub mod core;
pub mod state;
pub mod web;


#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
        <svg viewBox="0 0 120 120" class="myriadSVG">

        <BoardSVG />
        <RecentWords/>
        </svg>
        <FoundWordsTable/>
        <NewGameButton/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
