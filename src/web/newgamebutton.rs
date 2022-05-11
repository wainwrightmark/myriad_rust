use crate::web::prelude::*;
use crate::state::msg::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NewGameButtonProperties {
    pub selected_tab: usize,
}

#[function_component(NewGameButton)]
pub fn new_game_button(properties: &NewGameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| Msg::NewGame));

    let (x, y) = get_found_word_position(101, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"new_game_button"} text={"New Game"} {x} {y} width_units={3.5} {rect_class} {text_class} {on_click} />)
}

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProperties {
    pub total_found: usize,
    pub selected_tab: usize,
}

#[function_component(ScoreCounter)]
pub fn score_counter(properties: &ScoreCounterProperties) -> Html {
    let (x, y) = get_found_word_position(105, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-success");
    let text_class = classes!("button-text");


    html!(<FoundWordBox id={"score_counter"} text={format!("{:0>2}%", properties.total_found)} {x} {y} width_units={1.5} {rect_class} {text_class} />)
}

#[function_component(RotateButton)]
pub fn rotate_button(properties: &NewGameButtonProperties) -> Html{
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| Msg::FlipAndRotateRelative { rotate: 1, flip: false }));

    let (x, y) = get_found_word_position(111, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"rotate_button"} text={"Rotate"} {x} {y} width_units={3.5} {rect_class} {text_class} {on_click} />)
}

#[function_component(FlipButton)]
pub fn flip_button(properties: &NewGameButtonProperties) -> Html{
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| Msg::FlipAndRotateRelative { rotate: 0, flip: true }));

    let (x, y) = get_found_word_position(115, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"flip_button"} text={"Flip"} {x} {y} width_units={3.5} {rect_class} {text_class} {on_click} />)
}
