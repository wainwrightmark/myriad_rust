use crate::state::full_game_state::FullGameState;
use crate::state::game_size::*;
use crate::state::msg;
use crate::state::preferences_state::DarkModeNextMessage;
use crate::state::preferences_state::DarkModeState;
use crate::state::prelude::*;
use crate::web::prelude::*;

use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameButtonProperties {
    pub selected_tab: usize,

    pub position_number: i32,
    pub width: f32,
}

#[function_component(TodayGameButton)]
pub fn todays_game_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let navigator = use_navigator().unwrap();

    let func = move |_| {
        msg::move_to_new_game(true, &navigator);
    };
    let on_click: Option<Callback<MouseEvent>> = Some(Callback::<MouseEvent>::from(func));
    //Some(Dispatch::new().apply_callback(|_| NewGameMsg { today: true }));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"today_game_button"} text={"Today's Game"} {x} {y} width_units={properties.width}  {on_click} />)
}

#[function_component(RandomGameButton)]
pub fn random_game_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let navigator = use_navigator().unwrap();
    let on_click: Option<Callback<MouseEvent>> = Some(Callback::<MouseEvent>::from(move |_| {
        msg::move_to_new_game(false, &navigator);
    }));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"random_game_button"} text={"Random Game"} {x} {y} width_units={properties.width}  {on_click} />)
}

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProperties {
    pub selected_tab: usize,
    pub position_number: i32,
    pub width: f32,
}

#[function_component(ScoreCounter)]
pub fn score_counter(properties: &ScoreCounterProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    let (found, total) = *use_selector(|state: &FullGameState| state.get_found_count());
    let found_pc = found * 100 / total;
    let gradient = format!("background: linear-gradient(to right, var(--progress) {found_pc}%, var(--progress-blank) {found_pc}%, var(--progress-blank));");

    let width = format!("{}", FOUND_WORD_WIDTH * properties.width);
    let height = format!("{FOUND_WORD_HEIGHT}");
    let style = format!("position:absolute; transform: translate({x}px, {y}px); height: {height}px; width: {width}px; border-radius:5px; {gradient}");

    let class = classes!("found-word",);
    let key = "score_counter";


    let text = match total{
        100=> format_number(found as i32),
        _=> format!("{} / {}", found, total)
    } ;

    html!(
        <button {key} {style} {class} >
            {text}
        </button>
    )
}

#[function_component(DarkModeButton)]
pub fn dark_mode_button(properties: &GameButtonProperties) -> Html {
    use DarkModeState::*;
    let game_size = use_store_value::<GameSize>();
    let dark_mode_state = use_store_value::<DarkModeState>();

    let text = match *dark_mode_state {
        Auto => "🌒",
        Light => "☀️",
        Dark => "🌑",
    };

    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| DarkModeNextMessage));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"dark_mode_button"} {text} {x} {y} width_units={properties.width}  {on_click} />)
}

#[function_component(RotateButton)]
pub fn rotate_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg {
            rotate: myriad::prelude::QuarterTurns::One,
            flip: false,
        }));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"rotate_button"} text={"⟳"} {x} {y} width_units={properties.width}  {on_click} />)
}

#[function_component(FlipButton)]
pub fn flip_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg {
            rotate: myriad::prelude::QuarterTurns::Zero,
            flip: true,
        }));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"flip_button"} text={"⬌"} {x} {y} width_units={properties.width}  {on_click} />)
}

#[function_component(ShareButton)]
pub fn share_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let on_click: Callback<MouseEvent> = (|_: MouseEvent| {
        crate::web::sharing::share();
    })
    .into();

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"share_button"} text={"⠪"} {x} {y} width_units={properties.width}  {on_click} />)
}

#[function_component(HistoryButton)]
pub fn history_button(properties: &GameButtonProperties) -> Html {
    let game_size = use_store_value::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> = Some(
        Dispatch::<DialogState>::new()
            .reduce_mut_callback(|s| s.history_dialog_type = Some(Default::default())),
    );

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"history_button"} text={"H"} {x} {y} width_units={properties.width}  {on_click} />)
}


#[derive(PartialEq, Properties)]
pub struct ButtonBoxProperties {
    pub id: String,
    pub text: AttrValue,
    pub x: f32,
    pub y: f32,
    pub width_units: f32,
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(ButtonBox)]
fn button_box(properties: &ButtonBoxProperties) -> Html {
    let x = properties.x;
    let y = properties.y;
    let width = format!("{}", FOUND_WORD_WIDTH * properties.width_units);
    let height = format!("{FOUND_WORD_HEIGHT}");
    let style = format!("position:absolute; transform: translate({x}px, {y}px); height: {height}px; width: {width}px; border-radius:5px;");

    let class = classes!(
        "found-word",
        if properties.on_click.is_some() {
            Some("found-word-button")
        } else {
            None
        }
    );
    let key = properties.id.clone();

    html!(
        <button {key} {style} {class} onclick={properties.on_click.clone()}>
            {properties.text.clone()}
        </button>
    )
}
