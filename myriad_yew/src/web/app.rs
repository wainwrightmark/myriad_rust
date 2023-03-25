use crate::state::prelude::*;
use crate::web::found_words::*;
use crate::web::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;
use yewdux::prelude::Dispatch;
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

    let (width, height) = yew_hooks::use_size(node.clone());

    Dispatch::<GameSize>::new().apply(SetSizeMessage { width, height });

    let cheat = false;

    if width == 0 && height == 0{
        return html!(
            <div class="container" ref={node}/>
        );
    }

    html! {
        <>
        <CongratsDialog/>
        <HistoryDialog/>
        <div class="container" ref={node}>
        <Circles  />
        <Crosshairs />
        <TabHeaders />
        <AllFoundWords {cheat} />

        <RecentWords />

        <canvas id="confetti-canvas" class="confetti-canvas"></canvas>

        </div>
        </>
    }
}
