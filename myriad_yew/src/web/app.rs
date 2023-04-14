use crate::state::prelude::*;
use crate::web::found_words::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game/:game")]
    Game { game: String },
}

#[function_component(App)]
pub fn app() -> Html {
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
                .to_single_string();
            html! { <Redirect<Route> to={Route::Game { game  }} /> }
        }
        Route::Game { game } => {
            let cheat = false;
            html! { <MyriadApp {game} {cheat} />}
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
    let game = props.game.replace(' ', "+");
    let game = Game::from_string(game.as_str());
    if let Some(game) = game {
        Dispatch::new().apply(LoadGameMessage { game });
    }

    let node = use_node_ref();
    let (width, height) = yew_hooks::use_size(node.clone());

    Dispatch::<GameSize>::new().apply(SetSizeMessage { width, height });

    if width == 0 && height == 0 {
        return html!(
            <div class="outer-container">
            <div class="container" ref={node}/>
            </div>
        );
    }

    let size = GameSize {
        width: width as f32,
        height: height as f32,
    };

    html! {
        <>
        <CongratsDialog/>
        <HistoryDialog/>
        <div class="outer-container">
            <div class="container" ref={node} style={size.style_string()}>
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
