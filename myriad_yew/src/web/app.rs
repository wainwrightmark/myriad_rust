use crate::state::prelude::*;
use crate::web::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;
use crate::web::found_words::*;
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

    let node = use_node_ref();


    let (mut width,mut height) = yew_hooks::use_size(node.clone());
    if width ==0 && height == 0{
        width = 400;
        height = 400;
    }
    let (width, height) = (width as f32, height as f32);
    let game_size = GameSize{width, height};
    // let cheat = match route {
    //     Route::Cheat => true,
    //     Route::Home => false,
    // };

    let cheat = false;

    html! {
        <>
        <CongratsDialog/>
        <HistoryDialog/>
        <div class="container" ref={node}>
        <Circles {width} {height} />
        <FoundWordsTabHeaders {width} {height}/>
        <AllFoundWords {game_size} {cheat} />

        <RecentWords {width} {height}/>

        <canvas id="confetti-canvas" class="confetti-canvas"></canvas>

        </div>
        </>
    }
}
