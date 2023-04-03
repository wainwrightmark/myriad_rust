use crate::state::prelude::*;
use crate::web::found_words::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yew_hooks::use_search_param;
use yewdux::prelude::Dispatch;

#[function_component(App)]
pub fn myriad_app() -> Html {
    let cheat = use_search_param("cheat".to_string()).map(|x|x.to_ascii_lowercase() =="true").unwrap_or_default();
    let node = use_node_ref();
    let (width, height) = yew_hooks::use_size(node.clone());

    Dispatch::<GameSize>::new().apply(SetSizeMessage { width, height });

    if width == 0 && height == 0 {
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
        <InfoBar/>
        <TabHeaders />
        <AllFoundWords {cheat} />

        <RecentWords />

        <canvas id="confetti-canvas" class="confetti-canvas"></canvas>

        </div>
        </>
    }
}
