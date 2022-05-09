use crate::state::fullstate::*;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::web::SQUARE_SIZE;

#[function_component(RecentWords)]
pub fn recent_words() -> Html {
    let recent_words_state = use_selector(|state: &FullState| state.recent_words.clone());
    let rot_flip = use_selector(|state: &FullState| state.rotflip.clone());

    let recent_words = 
    recent_words_state.recent_words
        .iter()
        .rev()
        .map(|word| {
            let key = format!("{}_({:?})", word.word, word.expiry_time);

            let (cx, cy) = rot_flip
                .get_location(&word.coordinate, SQUARE_SIZE);

            let style = format!("animation-duration: {}ms;", word.linger_duration_ms());

            let text_anchor = if word.coordinate.column == 0 {
                "start"
            } else if word.coordinate.column == rot_flip.max_coordinate.column {
                "end"
            } else {
                "middle"
            };

            html! {
                <text
                {key}
                fill={word.get_color()}
                class="foundWord"
                {style}
                pointer-events="none"
                
                x={format!("{}", cx)}
                y={format!("{}", cy)}
                dominant-baseline="text-bottom"
                text-anchor={text_anchor}>
                {word.word.clone()}

              </text>
            }
        })
        .collect::<Html>();

    html! {
        {recent_words}
    }
}
