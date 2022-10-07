use crate::state::msg::*;
use crate::state::prelude::{DialogState, RotFlipMsg};
use crate::web::prelude::*;
use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameButtonProperties {
    pub selected_tab: usize,

    pub position_number: i32,
    pub width: f64,
}

#[function_component(TodayGameButton)]
pub fn todays_game_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| NewGameMsg { today: true }));

    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"today_game_button"} text={"Today's Game"} {x} {y} width_units={properties.width} {rect_class} {text_class} {on_click} />)
}

#[function_component(RandomGameButton)]
pub fn random_game_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| NewGameMsg { today: false }));

    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"random_game_button"} text={"Random Game"} {x} {y} width_units={properties.width} {rect_class} {text_class} {on_click} />)
}

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProperties {
    pub total_found: usize,
    pub selected_tab: usize,

    pub position_number: i32,
    pub width: f64,
}

#[function_component(ScoreCounter)]
pub fn score_counter(properties: &ScoreCounterProperties) -> Html {
    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("score-counter-box");
    let text_class = classes!("button-text");

    html!(
        <>
        <rect class={"score-counter-progress"} style={format!("transform: translate({}px, {}px);", x, y)} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width * properties.total_found.to_f64().unwrap() / 100.0 )}>
        </rect>
        <FoundWordBox id={"score_counter"} text={format_number(properties.total_found as i32)} {x} {y} width_units={properties.width} {rect_class} {text_class} />

        </>
    )
}

#[function_component(RotateButton)]
pub fn rotate_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg {
            rotate: 1,
            flip: false,
        }));

    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"rotate_button"} text={"⟳"} {x} {y} width_units={properties.width} {rect_class} {text_class} {on_click} />)
}

#[function_component(FlipButton)]
pub fn flip_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RotFlipMsg {
            rotate: 0,
            flip: true,
        }));

    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"flip_button"} text={"⬌"} {x} {y} width_units={properties.width} {rect_class} {text_class} {on_click} />)
}

#[function_component(HistoryButton)]
pub fn flip_button(properties: &GameButtonProperties) -> Html {
    let on_click: Option<Callback<MouseEvent>> = Some(
        Dispatch::<DialogState>::new()
            .reduce_mut_callback(|s| s.history_dialog_type = Some(Default::default())),
    );

    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");

    html!(<FoundWordBox id={"history_button"} text={"H"} {x} {y} width_units={properties.width} {rect_class} {text_class} {on_click} />)
}

#[function_component(WainwrongButton)]
pub fn wainwrong_button(properties: &GameButtonProperties) -> Html {
    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let text_class = classes!("button-text");
    let class = classes!("found-word-group", "found-word-group-button");

    let style = format!("transform: translate({}px, {}px);", x, y);

    html!(
     <g key={"wainwrong_link"} {style} {class} >
     
     <a href="https://wainwrong.com/">
     <rect class={rect_class} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width)}>
     
     </rect>
     <text class={text_class}>
     {"𝕨"}
     </text>
     
     
     </a>
     </g>)
}

#[function_component(FacebookButton)]
pub fn facebook_button(properties: &GameButtonProperties) -> Html {
    let (x, y) =
        get_found_word_position(properties.position_number, properties.selected_tab, false);

    let rect_class = classes!("found-word-box", "found-word-box-button");
    let class = classes!("found-word-group", "found-word-group-button");

    let style = format!("transform: translate({}px, {}px);", x, y);

    html!(
        <g key={"share_link"} {style} {class} >
     
        <a href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwainwrightmark.github.io%2Fmyriad_rust">
        <rect class={rect_class} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width)}>
        
        </rect>

        <svg data-license="From https://github.com/twbs/icons - Licensed under MIT" 
        fill="currentColor" 
        height="24" 
        style="margin: 0.1em; display: initial;" 
        viewBox="0 0 16 16" 
        width="24" 
        xmlns="http://www.w3.org/2000/svg">
        <title>{"BootstrapFacebook"}</title>
        <path d={FACEBOOK_ICON_PATH}></path></svg>
        
        </a>
        </g>)
}
