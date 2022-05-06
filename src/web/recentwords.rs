use yew::prelude::*;
use yewdux::prelude::*;
use crate::state::fullstate::*;


#[function_component(RecentWords)]
pub fn recent_words() -> Html {
    let (state, _) = use_store::<FullState>();

    let recent_words = state
        .recent_words
        .recent_words
        .iter().rev()
        .map(|word| {
            let id = format!("{}_({})", word.word, word.coordinate);

            let (cx, cy) = state.game.get_location(&word.coordinate, crate::web::board:: SQUARE_SIZE);

            let style = format!("animation-duration: {}ms;", word.linger_duration_ms());

            let text_anchor = if word.coordinate.column == 0 {
                "start"
            } else if word.coordinate.column == state.game.board.columns {
                "end"
            } else {
                "middle"
            };

            html! {
                <text
                fill={word.get_color()}
                class="foundWord"
                style={style}
                pointer-events="none"
                id={id}
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
