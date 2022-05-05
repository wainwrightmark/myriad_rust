use crate::core::coordinate::*;
use crate::core::move_result::*;

use bounce::*;
use foundwordsstate::*;
use gamestate::*;
use recentwordstate::*;
use yew::prelude::*;

pub mod core;
pub mod foundwordsstate;
pub mod gamestate;
pub mod recentwordstate;

#[function_component(RecentWords)]
fn recent_words() -> Html {
    let game_state = use_atom::<Gamestate>();
    let recent_word_state = use_atom::<RecentWordState>();

    let recent_words = recent_word_state
        .recent_words
        .iter()
        .map(|word| {
            let id = format!("{}-({})", word.word, word.coordinate);

            let (cx, cy) = game_state.get_location(&word.coordinate, SQUARE_SIZE);

            let style = format!("animation-duration: {}ms;", word.linger_duration_ms());

            let text_anchor = if word.coordinate.column == 0 {
                "start"
            } else if word.coordinate.column == game_state.board.columns {
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

#[function_component(FoundWordsTableContent)]
fn found_words_table_content() -> Html {

    let found_words_state = use_atom::<FoundWordsState>();

    let tab_content = (0..5).map(|twenties| {

        let chips = (0..20).map(|units|{
            let i = twenties * 20 + units;

            if i == 0{
                html!(<span class="label chip"></span>)
            }
            else {
                let found = found_words_state.words.contains_key(&i);

            if found{
                html!(<span class="label success chip">{i}</span>)
            }
            else{
                html!(<span class="label chip">{i}</span>)
            }
            }

            
            

        }).collect::<Html>();

        html! {
            <div>
            {chips}
            </div>
        }
    }).collect::<Html>();

    html! {
        {tab_content}
    }

}

#[function_component(FoundWordsTable)]
fn found_words_table() -> Html {
    

    let tab_labels = (0..5).map(|twenties| {
        let id = format!("tab-{twenties}");
        let label = (twenties * 20).to_string();

        html! {
            <>
            <input id={id.to_string()} type="radio" name="tabgroupB" />
            <label class="pseudo button toggle" for={id}>{label}</label>
            </>
        }
    }).collect::<Html>();

   
    

    html! {
        <div>
        <div class="tabs five">
        {tab_labels}
        <div class="row">
        <FoundWordsTableContent/>
        </div>
        </div>
        </div>
    }
}

const SQUARE_SIZE: f64 = 40.0;

#[function_component(BoardSVG)]
fn board_svg() -> Html {
    let game_state = use_atom::<Gamestate>();
    let found_words_state = use_atom::<FoundWordsState>();
    let recent_words_state = use_atom::<RecentWordState>();

    let rope_d = game_state.get_path_data(SQUARE_SIZE);

    let opacity = if game_state.chosen_positions.is_empty() {
        "0"
    } else {
        "!"
    };

    let circles = game_state
        .board
        .max_coordinate()
        .get_positions_up_to()
        .map(|c| {
            make_circle(
                &game_state,
                c,
                game_state.clone(),
                found_words_state.clone(),
                recent_words_state.clone(),
            )
        })
        .collect::<Html>();

    html! {
          <g>
                  <rect x="0" y="0" width="120" height="120" fill="white"/>
                  <path
    id="rope"

    style="stroke-width: 18; stroke: LightBlue; -webkit-transition: 1s ease-out; transition: 1s ease-out; fill: none;"
    stroke-linejoin="round"
    stroke-linecap="round"
    opacity={opacity}
    d={rope_d}
    />

    {circles}
    </g>

      }
}

fn make_circle(
    gamestate: &Gamestate,
    coordinate: Coordinate,
    game_state: UseAtomHandle<Gamestate>,
    found_words_state: UseAtomHandle<FoundWordsState>,
    recent_words_state: UseAtomHandle<RecentWordState>,
) -> Html {
    let location = gamestate.get_location(&coordinate, SQUARE_SIZE);
    let cx = location.0;
    let cy = location.1;
    let color = gamestate.get_color(&coordinate).to_string();
    let letter = gamestate.board.get_letter_at_coordinate(&coordinate);
    let text = letter.word_text();
    let cursor = "default";
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let radius = format!("{:2}", SQUARE_SIZE * 0.4);

    let g_style = format!(
        "-webkit-transform: translate({cx}px, {cy}px); transform: translate({cx}px, {cy}px);"
    );

    let on_click_callback = Callback::from(move |_: MouseEvent| {
        let move_result = game_state.get_move_result(&coordinate);
        let new_state = game_state.after_move_result(&move_result);
        game_state.set(new_state);

        let mut is_new_word: bool = false;

        if let MoveResult::WordComplete {
            word: found_word,
            coordinates: _,
        } = move_result.clone()
        {
            is_new_word = !found_words_state.has_word(&found_word);
            if is_new_word {
                found_words_state.set(found_words_state.with_word(found_word));
            }
        }

        recent_words_state.set(recent_words_state.after_move_result(&move_result, is_new_word));
    });

    html! {
        <g class="square"
       style={g_style}
       cursor={cursor}
       onclick={on_click_callback}
       >
      <circle
        id={circle_id}
        class="circle"
        stroke={color}
        fill="black"
        fill-opacity="0.01"
        r={radius}
        >
      </circle>

      <text
        id={text_id}
        class="circle-text"
        dominant-baseline="middle"
        text-anchor="middle"
        stroke="@Colors.Shades.White"
        fill="@Colors.Shades.Black">
        {text}
      </text>
    </g>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BounceRoot>
        <div class="container">
        <svg viewBox="0 0 120 120" class="myriadSVG">

        <BoardSVG />
        <RecentWords/>
        </svg>
        <FoundWordsTable/>
        </div>




        </BounceRoot>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
