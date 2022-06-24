use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let width = format!("{SVG_WIDTH}");
    let height = format!("{SVG_HEIGHT}");
    

    let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    html! {

        <>
        <Dialog/>
        
        <div class="container">
        <svg viewBox={view_box} class="myriadSVG" {onpointerup} >
        <rect x="0" y="0" {width} {height} fill="white"  />
        <CrosshairsSVG/>
        <CirclesSVG />


        <FoundWordsTabHeaders/>
        <AllFoundWords />

        <RecentWords/>
        </svg>



        <canvas id="confetti-canvas" style="position: fixed; width: 100%; height: 100%; top: 0px; left: 0px; z-index: 1000; pointer-events: none;"></canvas>

        </div>
        </>

    }
}
