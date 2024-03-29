use crate::state::preferences_state::DarkModeState;
use crate::state::prelude::*;
use crate::web::found_words::*;
use crate::web::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use yewdux::prelude::Dispatch;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,

    #[at("/history")]
    History,

    #[at("/game/:game")]
    Game { game: String },

    #[at("/cheat/:game")]
    Cheat { game: String },
}

#[function_component(App)]
pub fn app() -> Html {
    //Load the dark mode state here to make sure dark mode is set correctly
    let _dms = use_store::<DarkModeState>();

    use_effect_once(|| {
        spawn_local(crate::web::startup::setup());
        || ()
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            let game = Dispatch::<FullGameState>::new()
                .get()
                .game
                .board
                .canonical_string();
            html! { <Redirect<Route> to={Route::Game { game  }} /> }
        }
        Route::Game { game } => {
            let cheat = false;
            html! { <MyriadApp {game} {cheat} />}
        }

        Route::Cheat { game } => {
            let cheat = true;
            html! { <MyriadApp {game} {cheat} />}
        }

        Route::History {} => {
            html!( <HistoryPage/>)
        }
    }
}

#[derive(Debug, Properties, PartialEq, Clone)]
struct MyriadAppProps {
    pub game: String,
    pub cheat: bool,
}

#[function_component(MyriadApp)]
fn myriad_app(props: &MyriadAppProps) -> Html {
    let cheat = props.cheat;
    let game_str = url_replace(&props.game);
    let game = Game::from_string(game_str.as_str());

    if let Some(game) = game {
        Dispatch::new().apply(LoadGameMessage { game});
    }

    let (width, height) = yew_hooks::use_window_size();
    let (width, height) = (width as f32, height as f32);

    Dispatch::<GameSize>::new().apply(SetSizeMessage { width, height });

    if width == 0. && height == 0. {
        return html!(
            <div class="outer-container">
            <div class="container" />
            </div>
        );
    }

    let size = GameSize::from_width_and_height(width, height);

    let outer_container_style = size.outer_container_style();
    let container_style = size.container_style();

    html! {
        <>
        <CongratsDialog/>
        <div class="outer-container"  style={outer_container_style}>
            <div class="container"   style={container_style}>
                <Circles  />
                <Crosshairs />
                <InfoBar/>
                <TabHeaders />
                <AllFoundWords {cheat} />

                <RecentWords />

                <canvas id="confetti-canvas" class="confetti-canvas"></canvas>

            </div>
        </div>
        </>
    }
}

fn url_replace(s: &str) -> String {
    s.replace(' ', "+")
        .replace("%C3%B7", "÷")
        .replace("%C3%97", "×")
}
