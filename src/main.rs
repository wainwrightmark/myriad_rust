use yew::prelude::*;

use crate::web::prelude::*;
pub mod core;
pub mod state;
pub mod web;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">

        <NewGameButton/>
        <svg viewBox="0 0 120 120" class="myriadSVG">
        <rect x="0" y="0" width="120" height="120" fill="white"/>
        <RopeSVG />
        <CirclesSVG />
        
        <RecentWords/>
        </svg>
        <FoundWordsTable/>
        

        <canvas id="confetti-canvas" style="position: fixed; width: 100%; height: 100%; top: 0px; left: 0px; z-index: 1000; pointer-events: none;"></canvas>

        </div>


    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
