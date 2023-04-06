use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(RecentWords)]
pub fn recent_words() -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let recent_words_state = use_store_value::<RecentWordState>();
    let rot_flip = use_store_value::<RotFlipState>();
    let selected_index = *use_selector(|state: &SelectedTabState| state.index).deref();

    let recent_words = recent_words_state.recent_words
        .iter()
        .rev()
        .map(|word| {
            let key = format!("{}_({:?})", word.number, word.expiry_time);

            let mut start_location = rot_flip
                .get_location(&word.coordinate, game_size.as_ref());

            if word.coordinate.x() == 2{
                start_location.x *= 0.8; //little hack to prevent large numbers from being offscreen
            }

            let(endx, endy) = game_size.get_found_word_position(word.number, selected_index, true);

            let style = format!(
                "animation-duration: {}ms; --startx: {}px; --starty: {}px; --endx: {}px; --endy: {}px; color: {}",
             word.linger_duration_ms(),
             start_location.x,
             start_location.y,
             endx,
             endy,
             word.get_color()
            );

            //word.word
            let text = format_number(word.number);

            html! {
                <button
                {key}
                class="recent-word"
                {style}
                >
                {text}

            </button>
            }
        })
        .collect::<Html>();

    html! {
        <div class="recent-words">
        {recent_words}
        </div>
    }
}
