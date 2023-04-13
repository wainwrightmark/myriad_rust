use crate::state::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(HistoryDialog)]
pub fn history_dialog() -> Html {
    let (state, dispatch) = use_store::<DialogState>();

    let onclick = dispatch.reduce_mut_callback(|state| state.history_dialog_type = None);

    let (history, _) = use_store::<HistoryState>();

    if state.history_dialog_type.is_some() {
        let rows: Vec<Html> = history
            .games
            .iter()
            .map(|(game, words)| html!(<HistoryRow game={game.clone()} words={words.len()} />))
            .collect();

        html!(<dialog style="top: 10%" open={true}>
        <p>{"History"}</p>
        <div style="overflow-y:auto; height:300px;">
      <table>
      {rows}
      </table>
      </div>
      <button {onclick}>{"Ok"}</button>
      </dialog>)
    } else {
        html!(<></>)
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct HistoryRowProperties {
    pub game: Game,
    pub words: usize,
}

#[function_component(HistoryRow)]
pub fn history_row(properties: &HistoryRowProperties) -> Html {
    let game = properties.game.clone();
    let onclick = Dispatch::<FullGameState>::new()
        .apply_callback(move |_| LoadGameMessage { game: game.clone() });

    let style = format!("width: {}%;", properties.words);

    let date = properties
        .game
        .date
        .map(|x| x.to_string())
        .unwrap_or_default();

    html!(<tr>
      <td><button {onclick}>{properties.game.board.to_single_string()}</button> </td>
    <td>
    {date}
    </td>
    <td>
    <div class="history-progress-box">
    <div class="history-progress" style={style}>{properties.words}</div>
    </div>
    </td>
    
     </tr>)
}

#[function_component(CongratsDialog)]
pub fn congrats_dialog() -> Html {
    let (state, dispatch) = use_store::<DialogState>();

    let on_ok = dispatch.reduce_mut_callback(|state| state.congratulations_dialog_type = None);
    let on_share = dispatch.reduce_mut_callback(|state| {
        state.congratulations_dialog_type = None;
        crate::web::sharing::share();
    });

    if let Some(dialog_type) = state.congratulations_dialog_type {
        let message: &str = match dialog_type {
            CongratsDialogType::OneHundred => {
                "Well done, you got 💯!"
            }
        };
        html!(
            <dialog style="top: 25%" open={true}>
                <p>{message}</p>
        <form>
      <button onclick={on_ok}>{"Ok"}</button>
      <button onclick={on_share}>{"Share"}</button>
    
    </form>
      </dialog>)
    } else {
        html!(<></>)
    }
}
