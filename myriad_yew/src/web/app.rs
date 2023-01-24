use crate::state::prelude::*;
use crate::web::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;
use yewdux::prelude::*;

// #[derive(Clone, Routable, PartialEq)]
// enum Route {
//     #[at("/Cheat")]
//     Cheat,
//     #[at("/")]
//     Home,
// }

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RouteQuery {
    #[serde(default)]
    pub cheat: bool,
}

// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <BrowserRouter>
//             <Switch<Route> render={myriad_app} />
//         </BrowserRouter>
//     }
// }

#[function_component(App)]
pub fn myriad_app(//route : Route
) -> Html {
    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let width = format!("{SVG_WIDTH}");
    let height = format!("{SVG_HEIGHT}");

    // let cheat = match route {
    //     Route::Cheat => true,
    //     Route::Home => false,
    // };

    let cheat = false;



    html! {

        <>
        <CongratsDialog/>
        <HistoryDialog/>
        <div class="container" style="position:relative;">
        <Circles />
        <FoundWordsControl/>
        // <svg viewBox={view_box} class="myriadSVG">
        // <rect x="0" y="0" {width} {height} fill="white"  />




        // <FoundWordsTabHeaders/>
        // <AllFoundWords {cheat} />

        // <RecentWords/>
        // </svg>



        <canvas id="confetti-canvas" style="position: fixed; width: 100%; height: 100%; top: 0px; left: 0px; z-index: 1000; pointer-events: none;"></canvas>

        </div>
        </>

    }
}


// #[function_component(App)]
// pub fn game_area()-> Html{
//     let state = use_size(node.clone());



// }