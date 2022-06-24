use crate::state::msg::*;
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
    let (x, y) = get_found_word_position(107, properties.selected_tab, false);

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

// #[function_component(RotateButton)]
// pub fn rotate_button(properties: &NewGameButtonProperties) -> Html{
//     let on_click: Option<Callback<MouseEvent>> =
//         Some(Dispatch::new().apply_callback(|_| Msg::FlipAndRotateRelative { rotate: 1, flip: false }));

//     let (x, y) = get_found_word_position(111, properties.selected_tab, false);

//     let rect_class = classes!("found-word-box", "found-word-box-button");
//     let text_class = classes!("button-text");

//     html!(<FoundWordBox id={"rotate_button"} text={"Rotate"} {x} {y} width_units={3.5} {rect_class} {text_class} {on_click} />)
// }

// #[function_component(FlipButton)]
// pub fn flip_button(properties: &NewGameButtonProperties) -> Html{
//     let on_click: Option<Callback<MouseEvent>> =
//         Some(Dispatch::new().apply_callback(|_| Msg::FlipAndRotateRelative { rotate: 0, flip: true }));

//     let (x, y) = get_found_word_position(115, properties.selected_tab, false);

//     let rect_class = classes!("found-word-box", "found-word-box-button");
//     let text_class = classes!("button-text");

//     html!(<FoundWordBox id={"flip_button"} text={"Flip"} {x} {y} width_units={3.5} {rect_class} {text_class} {on_click} />)
// }
