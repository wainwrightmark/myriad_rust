use crate::{state::{ prelude::*}, web::prelude::{Route, format_number}};
use chrono::Duration;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let navigator = use_navigator().unwrap();
    let history = use_store_value::<HistoryState>();

    let current_game = use_store_value::<FullGameState>();

    let onclick: Callback<MouseEvent> =
    Callback::from(move |_me:MouseEvent| navigator.push(&Route::Home));


    let rows: Vec<Html> = history
        .all_games_including_current(current_game.as_ref())
            // .games
            // .iter()
            // .rev()
            .map(|state| html!(<HistoryRow state={state.clone()} />))
            .collect();

    html!(<div class="history-page">
        <p class="myriad-logo" {onclick}>{"ↂ"}</p>
        <p class="page-header">{"History"}</p>
        <br/>
      <table class="history-table">
      {rows}
      </table>
      </div>)
}

#[derive(PartialEq, Eq, Properties)]
pub struct HistoryRowProperties {
    pub state: FullGameState
}

#[function_component(HistoryRow)]
pub fn history_row(properties: &HistoryRowProperties) -> Html {
    let navigator = use_navigator().unwrap();
    let game = properties.state.game.clone();
    let onclick: Callback<MouseEvent> =
    Callback::from(move |_me:MouseEvent| navigator.push(&Route::Game{game: game.board.canonical_string()}));

    let (found, total) = properties.state.get_found_count();
    let found_pc = found * 100 / total;
    let gradient_to = "right";
    let gradient = format!("background: linear-gradient(to {gradient_to}, var(--progress) {found_pc}%, var(--progress-blank) {found_pc}%, var(--progress-blank));");

    let progress_text = match total {
        100 => format_number(found as i32),
        _ => format!("{} / {}", found, total),
    };


    let style = format!("{gradient}");

    let date = properties.state
        .game
        .date
        .map(|x| x.to_string())
        .unwrap_or_default();

    let time = match properties.state.timing{
        GameTiming::Started { .. } => html!(<td></td>),
        GameTiming::Finished { total_milliseconds } => {
            let total = Duration::milliseconds(total_milliseconds as i64);
                if total.num_minutes() >= 100 {
                    html!(<></>)
                } else {
                    let minutes = total.num_minutes();
                    let seconds = total.num_seconds() - (60 * minutes);
                    let time_string = format!("{minutes:02}:{seconds:02}");
                    html!(<td>
                        <p class="history-time-display">{time_string}</p>
                         </td>)
                }
        },
        GameTiming::Cheat => html!(<td><p class="history-time-display" >{"😈"}</p></td>),
        GameTiming::Unknown => html!(<td></td>),
    };

    html!(<tr>
      <td><button {onclick} class="button-text">{properties.state .game.board.to_single_string()}</button> </td>
    <td>
    {date}
    </td>
    <td>
    <div class="history-progress-box">
    <div class="history-progress" style={style}>{progress_text}</div>
    </div>
    </td>
        {time}
     </tr>)
}
