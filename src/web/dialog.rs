use crate::state::prelude::*;
use crate::web::prelude::*;
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

    let onclick = dispatch.reduce_mut_callback(|state| state.congratulations_dialog_type = None);

    if let Some(dialog_type) = state.congratulations_dialog_type {
        let message: &str;
        //let quote: &str;

        match dialog_type {
            CongratsDialogType::Challenge => {
                message = "Well done, you beat challenge mode!\r\nNow try for 💯!";
                //quote = "I%20beat%20challenge%20mode%20in%20myriad%21";
            }
            CongratsDialogType::OneHundred => {
                message = "Well done, you got 💯!";
                //quote = "I%20got%20%F0%9F%92%AF%20in%20myriad%21"
            }
        }

        let link =
            "https://www.facebook.com/sharer/sharer.php?u=wainwrightmark.github.io%2Fmyriad_rust";
        html!(<dialog style="top: 25%" open={true}>
        <p>{message}</p>
        <form>
      <button formaction={link}>
      
      <svg data-license="From https://github.com/twbs/icons - Licensed under MIT" 
        fill="currentColor" 
        height="24" 
        style="margin: 0.1em; display: initial;" 
        viewBox="0 0 16 16" 
        width="24" 
        xmlns="http://www.w3.org/2000/svg">
        <title>{"BootstrapFacebook"}</title>
        <path d={FACEBOOK_ICON_PATH}></path></svg>
      </button>
      <button {onclick}>{"Ok"}</button>
    
    </form>
      </dialog>)
    } else {
        html!(<></>)
    }
}
