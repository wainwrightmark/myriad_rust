use crate::state::full_game_state::FullGameState;
use crate::state::game_size::*;
use crate::state::msg;
use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameButtonProperties {
    pub selected_tab: usize,

    pub position_number: i32,
    pub width: f32,
}

#[function_component(TodayGameButton)]
pub fn todays_game_button(properties: &GameButtonProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> = Some(Callback::<MouseEvent>::from(|_| {msg::move_to_new_game(true);}));
        //Some(Dispatch::new().apply_callback(|_| NewGameMsg { today: true }));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"today_game_button"} text={"Today's Game"} {x} {y} width_units={properties.width} color="white" {on_click} />)
}

#[function_component(RandomGameButton)]
pub fn random_game_button(properties: &GameButtonProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> = Some(Callback::<MouseEvent>::from(|_| {msg::move_to_new_game(false);}));

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"random_game_button"} text={"Random Game"} {x} {y} width_units={properties.width} color="white" {on_click} />)
}

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProperties {
    pub total_found: usize,
    pub selected_tab: usize,

    pub position_number: i32,
    pub width: f32,
}

#[function_component(ScoreCounter)]
pub fn score_counter(properties: &ScoreCounterProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    let (found,total) = *use_selector(|state: &FullGameState| state.get_found_count());
    let found_pc = found * 100 / total;
    let gradient = format!("background: linear-gradient(to right, green {found_pc}%, lightgrey {found_pc}%, lightgrey);");

    let width = format!("{}", FOUND_WORD_WIDTH * properties.width);
    let height = format!("{FOUND_WORD_HEIGHT}");
    let style = format!("position:absolute; transform: translate({x}px, {y}px); height: {height}px; width: {width}px; border-radius:5px; {gradient}");

    let class = classes!("found-word",);
    let key = "score_counter";

    let text = format_number(properties.total_found as i32);

    html!(
        <button {key} {style} {class} >
            {text}
        </button>
    )
}

#[function_component(RotateButton)]
pub fn rotate_button(properties: &GameButtonProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
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

    html!(<ButtonBox id={"rotate_button"} text={"⟳"} {x} {y} width_units={properties.width} color="white" {on_click} />)
}

#[function_component(FlipButton)]
pub fn flip_button(properties: &GameButtonProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
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

    html!(<ButtonBox id={"flip_button"} text={"⬌"} {x} {y} width_units={properties.width} color="white" {on_click} />)
}

#[function_component(HistoryButton)]
pub fn history_button(properties: &GameButtonProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let on_click: Option<Callback<MouseEvent>> = Some(
        Dispatch::<DialogState>::new()
            .reduce_mut_callback(|s| s.history_dialog_type = Some(Default::default())),
    );

    let (x, y) = game_size.get_found_word_position(
        properties.position_number,
        properties.selected_tab,
        false,
    );

    html!(<ButtonBox id={"history_button"} text={"H"} {x} {y} width_units={properties.width} color="white" {on_click} />)
}

// #[function_component(WainwrongButton)]
// pub fn wainwrong_button(properties: &GameButtonProperties) -> Html {
//     let (x, y) =properties.game_size.
//         get_found_word_position(properties.position_number, properties.selected_tab, false);

//     let rect_class = classes!("found-word-box", "found-word-box-button");
//     let text_class = classes!("button-text");
//     let class = classes!("found-word", "found-word-button");

//     let style = format!("transform: translate({x}px, {y}px);");

//     html!(
//      <g key={"wainwrong_link"} {style} {class} >

//      <a href="https://wainwrong.com/">
//      <rect class={rect_class} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width)}>

//      </rect>
//      <text class={text_class}>
//      {"𝕨"}
//      </text>

//      </a>
//      </g>)
// }

// #[function_component(FacebookButton)]
// pub fn facebook_button(properties: &GameButtonProperties) -> Html {
//     let (x, y) =properties.game_size.
//         get_found_word_position(properties.position_number, properties.selected_tab, false);

//     let rect_class = classes!("found-word-box", "found-word-box-button");
//     let class = classes!("found-word", "found-word-button");

//     let style = format!("transform: translate({x}px, {y}px);");

//     html!(
//         <g key={"share_link"} {style} {class} >

//         <a href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwainwrightmark.github.io%2Fmyriad_rust">
//         <rect class={rect_class} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width)}>

//         </rect>

//         <svg data-license="From https://github.com/twbs/icons - Licensed under MIT"
//         fill="currentColor"
//         height="24"
//         style="margin: 0.1em; display: initial;"
//         viewBox="0 0 16 16"
//         width="24"
//         xmlns="http://www.w3.org/2000/svg">
//         <title>{"BootstrapFacebook"}</title>
//         <path d={FACEBOOK_ICON_PATH}></path></svg>

//         </a>
//         </g>)
// }

#[derive(PartialEq, Properties)]
pub struct ButtonBoxProperties {
    pub id: String,
    pub text: AttrValue,
    pub color: AttrValue,
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
    let color = &properties.color;
    let style = format!("position:absolute; transform: translate({x}px, {y}px); height: {height}px; width: {width}px; border-radius:5px; background-color: {color};");

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
