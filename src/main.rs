use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::coordinate::*;
use crate::core::creator::*;
use crate::core::move_result::*;
use crate::core::solver::*;

use foundwordsstate::*;
use yew::prelude::*;
use yewdux::prelude::*;

use gamestate::*;
use recentwordstate::*;

use serde::*;


pub mod core;
pub mod foundwordsstate;
pub mod gamestate;
pub mod recentwordstate;

#[derive(PartialEq, Store, Clone,  Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"

pub struct FullState {
    pub game: Rc<Gamestate>,
    pub found_words: Rc<FoundWordsState>,
    #[serde(skip)]
    pub recent_words: Rc<RecentWordState>,
}


enum Msg {
    NewGame,
    Move { coordinate: Coordinate },
}

impl Reducer<FullState> for Msg {
    fn apply(&self, state: Rc<FullState>) -> Rc<FullState> {
        match self {
            Msg::NewGame => {
                let solver = Solver {
                    settings: SolveSettings { min: 1, max: 100 },
                };

                let settings = BoardCreateSettings {
                    branches_to_take: 2,
                    desired_solutions: 100,
                    number_to_return: 1,
                };
                let rng = rand::SeedableRng::seed_from_u64(10);
                let rng_cell = RefCell::new(rng);

                let boards = crate::core::creator::create_boards(&solver, 9, &settings, &rng_cell);

                let new_game_state = Gamestate {
                    board: boards[0].clone(),
                    ..Default::default()
                };

                FullState {
                    game: new_game_state.into(),
                    recent_words: Default::default(),
                    found_words: Default::default(),
                }
                .into()
            }
            Msg::Move { coordinate } => {
                let move_result = state.game.get_move_result(&coordinate);

                let new_game_state = state.game.deref().clone().after_move_result(&move_result);

                let mut is_new_word: bool = false;

                let new_found_words :Rc<FoundWordsState> = if let MoveResult::WordComplete {
                    word: found_word,
                    coordinates: _,
                } = move_result.clone()
                {
                    is_new_word = !state.found_words.has_word(&found_word);
                    if is_new_word {
                        state.found_words.with_word(found_word).into()
                    } else {
                        state.found_words.clone()
                    }
                } else {
                    state.found_words.clone()
                };

                let new_recent_words = state
                    .recent_words.deref().clone()
                    .after_move_result(&move_result, is_new_word);

                FullState {
                    game: new_game_state.into(),
                    recent_words: new_recent_words.into(),
                    found_words: new_found_words,
                }
                .into()
            }
        }
    }
}

#[function_component(NewGameButton)]
fn new_game_button() -> Html {
    let dispatch = Dispatch::<FullState>::new();

    let onclick = dispatch.apply_callback(|_| Msg::NewGame);

    html! {
        <div>
            <button {onclick }>{"New Game"} </button>
        </div>
    }
}

#[function_component(RecentWords)]
fn recent_words() -> Html {
    let (state, _) = use_store::<FullState>();

    let recent_words = state
        .recent_words
        .recent_words
        .iter().rev()
        .map(|word| {
            let id = format!("{}_({})", word.word, word.coordinate);

            let (cx, cy) = state.game.get_location(&word.coordinate, SQUARE_SIZE);

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

#[function_component(FoundWordsTableContent)]
fn found_words_table_content() -> Html {
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let tab_content = (0..5)
        .map(|twenties| {
            let chips = (0..20)
                .map(|units| {
                    let i = twenties * 20 + units;

                    if i == 0 {
                        html!(<span class="label chip"></span>)
                    } else {
                        let found = found_words_state.words.contains_key(&i);

                        if found {
                            html!(<span class="label success chip">{i}</span>)
                        } else {
                            html!(<span class="label chip">{i}</span>)
                        }
                    }
                })
                .collect::<Html>();

            html! {
                <div>
                {chips}
                </div>
            }
        })
        .collect::<Html>();

    html! {
        {tab_content}
    }
}

#[function_component(FoundWordsTable)]
fn found_words_table() -> Html {
    let tab_labels = (0..5)
        .map(|twenties| {
            let id = format!("tab-{twenties}");
            let label = (twenties * 20).to_string();            
            html! {
                <>
                <input id={id.to_string()} type="radio" name="tabgroupB" checked={twenties == 0} />
                <label class="pseudo button toggle" for={id}>{label}</label>
                </>
            }
        })
        .collect::<Html>();

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
    let game_state = use_selector(|state: &FullState| state.game.clone());

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
        .map(|c| make_circle(&game_state, c))
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

fn make_circle(gamestate: &Gamestate, coordinate: Coordinate) -> Html {
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

    let onclick = Dispatch::new().apply_callback(move |_| Msg::Move {
        coordinate: coordinate,
    });

    html! {
        <g class="square"
       style={g_style}
       cursor={cursor}
       {onclick}
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
        <div class="container">
        <svg viewBox="0 0 120 120" class="myriadSVG">

        <BoardSVG />
        <RecentWords/>
        </svg>
        <FoundWordsTable/>
        <NewGameButton/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
