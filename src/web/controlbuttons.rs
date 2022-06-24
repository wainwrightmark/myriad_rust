use crate::state::msg::*;
use crate::state::prelude::RotFlipMsg;
use crate::web::prelude::*;
use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameButtonProperties {
    pub selected_tab: usize,
}

#[function_component(TodayGameButton)]
pub fn todays_game_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| NewGameMsg {today: true}));

    let (x, y) = get_found_word_position(101, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"today_game_button"} text={"Today's Game"} {x} {y} width_units={6.0} {rect_class} {text_class} {on_click} />)
}

#[function_component(RandomGameButton)]
pub fn random_game_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| NewGameMsg {today: false}));

    let (x, y) = get_found_word_position(111, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"random_game_button"} text={"Random Game"} {x} {y} width_units={6.0} {rect_class} {text_class} {on_click} />)
}

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProperties {
    pub total_found: usize,
    pub selected_tab: usize,
}

#[function_component(ScoreCounter)]
pub fn score_counter(properties: &ScoreCounterProperties) -> Html {
    let (x, y) = get_found_word_position(106, properties.selected_tab, false);

    let rect_class = classes!("score-counter-box");
    let text_class = classes!("button-text");

    html!(
        <>
        <rect class={"score-counter-progress"} style={format!("transform: translate({}px, {}px);", x, y)} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * 1.5 * properties.total_found.to_f64().unwrap() / 100.0 )}>
        </rect>
        <FoundWordBox id={"score_counter"} text={format_number(properties.total_found as i32)} {x} {y} width_units={1.5} {rect_class} {text_class} />

        </>
    )
}

#[function_component(RotateButton)]
pub fn rotate_button(properties: &GameButtonProperties) -> Html{
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg{ rotate: 1, flip: false }));

    let (x, y) = get_found_word_position(108, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"rotate_button"} text={"⟳"} {x} {y} width_units={1.0} {rect_class} {text_class} {on_click} />)
}

#[function_component(FlipButton)]
pub fn flip_button(properties: &GameButtonProperties) -> Html{
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg{ rotate: 0, flip: true }));

    let (x, y) = get_found_word_position(109, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"flip_button"} text={"⮀"} {x} {y} width_units={1.0} {rect_class} {text_class} {on_click} />)
}

#[function_component(WainwrongButton)]
pub fn wainwrong_button(properties: &GameButtonProperties) -> Html{

    let (x, y) = get_found_word_position(119, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");
    let class = classes!("found-word-group", "found-word-group-button");

    let style = format!("transform: translate({}px, {}px);", x, y);

    html!(
     <g key={"wainwrong_link"} {style} {class} >
     
     <a href="https://wainwrong.com/">
     <rect class={rect_class} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * 1.0)}>
     
     </rect>
     <text class={text_class}>
     {"𝕨"}
     </text>
     
     
     </a>
     </g>)
}