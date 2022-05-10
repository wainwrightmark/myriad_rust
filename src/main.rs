use yew::prelude::*;

use crate::web::prelude::*;
use crate::web::SVG_HEIGHT;
use crate::web::SVG_WIDTH;
pub mod core;
pub mod state;
pub mod web;

#[function_component(App)]
fn app() -> Html {

    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let width = format!("{SVG_WIDTH}");
    let height = format!("{SVG_HEIGHT}");
    html! {
        <div class="container">
        <svg viewBox={view_box} class="myriadSVG">
        <rect x="0" y="0" {width} {height} fill="white"/>
        <RopeSVG />
        <CirclesSVG />

        
        <FoundWordsTabHeaders/>
        <AllFoundWords />

        <RecentWords/>
        </svg>
        


        <canvas id="confetti-canvas" style="position: fixed; width: 100%; height: 100%; top: 0px; left: 0px; z-index: 1000; pointer-events: none;"></canvas>

        </div>


    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
