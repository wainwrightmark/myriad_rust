use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(RecentWords)]
pub fn recent_words() -> Html {
    let recent_words_state = use_store_value::<RecentWordState>();
    let rot_flip = use_store_value::<RotFlipState>();
    let selected_index = *use_selector(|state: &SelectedTabState| state.index).deref();

    let recent_words = recent_words_state.recent_words
        .iter()
        .rev()
        .map(|word| {
            let key = format!("{}_({:?})", word.number, word.expiry_time);

            let (mut startx, starty) = rot_flip
                .get_location(&word.coordinate, SQUARE_SIZE);

            if word.coordinate.col() == 2{
                startx *= 0.8; //little hack to prevent large numbers from being offscreen
            }

            let(endx, endy) = crate::web::found_words::get_found_word_position(word.number, selected_index, true);

            let style = format!("animation-duration: {}ms; --startx: {}px; --starty: {}px; --endx: {}px; --endy: {}px;",
             word.linger_duration_ms(),
             startx,
             starty,
             endx + 2.5,
             endy + 5.0
            );

            //word.word
            let text = format_number(word.number);

            html! {
                <text
                {key}
                fill={word.get_color()}
                class="recentWord"
                {style}
                pointer-events="none"
                >
                {text}

              </text>
            }
        })
        .collect::<Html>();

    html! {
        {recent_words}
    }
}
