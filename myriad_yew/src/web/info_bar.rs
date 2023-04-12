use crate::state::info_bar_state::InfoBarState;
use crate::state::prelude::*;
use yew::function_component;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(InfoBar)]
pub fn info_bar() -> Html {
    let (state, _) = use_store::<InfoBarState>();
    let (size, _) = use_store::<GameSize>();

    let color = state.text_color();
    let font_size = state.font_size();
    let line_height = state.line_height();
    let (x, y) = size.get_info_bar_position();

    html! {
        <div class="infobar" style={format!("transform: translate({x}px, {y}px); color: {color}; font-size: {font_size}; line-height: {line_height};")}>
        <span class="infobar-text">
        {state.text()}
        </span>

        </div>
    }
}
