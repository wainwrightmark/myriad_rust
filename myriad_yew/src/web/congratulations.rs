use crate::state::{prelude::*, game_rating::{GameRating, SuboptimalWord}};
use chrono::Duration;
use myriad::prelude::{Board, FoundWord};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;



#[function_component(CongratsDialog)]
pub fn congrats_dialog() -> Html {
    let navigator = use_navigator().unwrap();
    let (dialog_state, dispatch) = use_store::<DialogState>();

    let on_ok = dispatch.reduce_mut_callback(|state| state.congratulations_dialog_type = None);
    let on_share = dispatch.reduce_mut_callback(|state| {
        state.congratulations_dialog_type = None;
        crate::web::sharing::share();
    });



    let on_new_game = dispatch.reduce_mut_callback(move |state| {
        state.congratulations_dialog_type = None;
        crate::state::msg::move_to_new_game(false, &navigator);
    });

    let timing = use_selector(|state: &FullGameState| state.timing.clone());

    let rating = use_selector(|state: &FullGameState| GameRating::create(state));
    let board = use_selector(|state: & FullGameState| state.game.board.clone());

    if let Some(dialog_type) = dialog_state.congratulations_dialog_type {
        let message: &str = match dialog_type {
            CongratsDialogType::OneHundred => "Well done, you got 💯!",
        };

        let time_box = match *timing {
            GameTiming::Cheat => html!(<>

                <p class="time-display">{"You Cheated!"}</p>
                 </>),
            GameTiming::Started { .. } | GameTiming::Unknown => html!(<></>),
            GameTiming::Finished { total_milliseconds } => {
                let total = Duration::milliseconds(total_milliseconds as i64);
                if total.num_minutes() >= 100 {
                    html!(<></>)
                } else {
                    let minutes = total.num_minutes();
                    let seconds = total.num_seconds() - (60 * minutes);
                    let time_string = format!("{minutes:02}:{seconds:02}");
                    html!(<>
                        <p class="time-display">{time_string}</p>
                         </>)
                }
            }
        };

        let rating_box = rating_box(&rating, &board);

        html!(
            <dialog style="top: 25%" open={true}>
                    <p class="dialog-message">{message}</p>
                    {time_box}
                    {rating_box}
                <div class="dialog-buttons">
                    <button class="dialog-button" onclick={on_ok}>{"Ok"}</button>
                    <button class="dialog-button" onclick={on_share}>{"Share"}</button>
                    <button class="dialog-button" onclick={on_new_game}>{"New Game"}</button>
                </div>
          </dialog>
        )
    } else {
        html!(<></>)
    }
}

fn rating_box(game_rating: &GameRating, board: &Board<GRID_ROWS, GRID_COLUMNS, GRID_SIZE>)->Html{

    let score_box =

    if game_rating.actual_steps == game_rating.min_steps{
        html!(
            <p class="score-display">{"Perfect Score"}</p>
        )
    }
    else{
        html!(
            <p class="score-display">{format!("{}/{}", game_rating.actual_steps, game_rating.min_steps)}</p>
        )
    };


    let suboptimal = if !game_rating.suboptimal_words.is_empty(){
        let ws: Html = game_rating.suboptimal_words.iter().map(|x| suboptimal_word(x, board)).collect();
        html!(
            <>
            <details>
                <summary class="suboptimal-summary">
                    {format!("{} Suboptimal {}", game_rating.suboptimal_words.len(), if game_rating.suboptimal_words.len() == 1 {"Solution"} else {"Solutions"})}
                    <span class="icon">{"↓"}</span>

                </summary>
                <div style="max-height: 200px; overflow-y:scroll;">
                <table class="suboptimal-table">
                {ws}
                </table>
                </div>
            </details>
            <br/>
            </>
        )
    } else{
        html!(<></>)
    };

    let hard = if !game_rating.hard_words.is_empty(){
        let ws: Html = game_rating.hard_words.iter().map(|x| hard_word(x, board)).collect();
        html!(
            <>
            <details>
                <summary class="hard-summary">
                    {format!("{} Hard {}", game_rating.hard_words.len(), if game_rating.suboptimal_words.len() == 1 {"Solution"} else {"Solutions"})}
                    <span class="icon">{"↓"}</span>

                </summary>
                {ws}
            </details>
            <br/>
            </>
        )
    } else{
        html!(<></>)
    };


    html!(
        <>
        {score_box}
        {suboptimal}
        {hard}
        </>
    )
}

fn suboptimal_word(w: &SuboptimalWord, board: &Board<GRID_ROWS, GRID_COLUMNS, GRID_SIZE> )-> Html{
    html!(
        <tr>
        <td>{w.result()}</td>
        <td>{w.actual.runes(board)}</td>
        <td>{w.best.runes(board)}</td>

        </tr>

    )
}

fn hard_word(w: &FoundWord<GRID_ROWS, GRID_COLUMNS, GRID_SIZE>, board: &Board<GRID_ROWS, GRID_COLUMNS, GRID_SIZE>)-> Html{
    html!(<p class="hardest-display">{format!("{} = {}", w.runes(board), w.result)}</p>)
}
