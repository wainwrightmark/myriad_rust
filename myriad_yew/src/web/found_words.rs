use std::rc::Rc;

use crate::state::found_words_state::FoundWordsState;
use crate::state::selected_tab_state::SelectedTabState;

use crate::state::prelude::*;
use crate::web::prelude::*;
use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FoundWordsTabHeaders)]
pub fn found_words_tab_headers(game_size: &GameSize) -> Html {
    let state = use_selector(|state: &FullGameState| state.found_words.clone());
    let selected_tab_state = use_store_value::<SelectedTabState>();
    let game_size = *game_size;

    let buttons = (0..5)
        .map(|index| {
            found_words_tab_header(index, game_size, state.clone(), selected_tab_state.clone())
        })
        .collect::<Html>();

    html!(<div class="tab-headers"> { buttons } <MoveTabHeader index={5} {selected_tab_state} {game_size}/> </div>)
}

#[derive(PartialEq, Properties)]
pub struct MoreTabHeaderProperties {
    index: usize,
    selected_tab_state: Rc<SelectedTabState>,
    game_size: GameSize,
}

#[function_component(MoveTabHeader)]
pub fn more_tab_header(properties: &MoreTabHeaderProperties) -> Html {
    let index = properties.index;
    let selected_tab_state = &properties.selected_tab_state;

    let onclick = Dispatch::new().apply_callback(move |_| TabSelectedMsg { index });

    let key = format!("found_words_tab_header{index}");
    let selected = if selected_tab_state.index == index {
        Some("selected-tab")
    } else if selected_tab_state.locked {
        Some("locked-out-tab")
    } else {
        None
    };

    let x = TAB_HEADER_PADDING + (index.to_f32().unwrap() * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN));
    let y = (properties.game_size.square_length() * 3.0) + TAB_HEADER_TOP_MARGIN;

    let class = classes!("tab-header", selected);
    let style = format!(
        "
        transform: translate({x}px, {y}px);",


    );

    html!(
        <button {class}  {style} {onclick} {key}>
           {"☰"}
        </button>
    )
}

pub fn found_words_tab_header(
    index: usize,
    game_size: GameSize,
    state: Rc<Rc<FoundWordsState>>,
    selected_tab_state: Rc<SelectedTabState>,
) -> Html {
    let onclick = Dispatch::new().apply_callback(move |_| TabSelectedMsg { index });

    let key = format!("found_words_tab_header{index}");
    let selected = if selected_tab_state.index == index {
        Some("selected-tab")
    } else if selected_tab_state.locked {
        Some("locked-out-tab")
    } else {
        None
    };
    let complete = if state.is_goal_complete(index) {
        Some("complete-tab")
    } else {
        None
    };

    let class = classes!("tab-header", selected, complete);
    let style = format!(
        "transform: translate({}px, {}px);",
        TAB_HEADER_PADDING + (index.to_f32().unwrap() * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN)),
        (game_size.square_length() * 3.0) + TAB_HEADER_TOP_MARGIN
    );

    html!(

        <button {class}  {style} {onclick} {key}>
        {format_number ((index.to_i32().unwrap()  + 1) * GOALSIZE)}
     </button>

    )
}

#[derive(PartialEq, Properties)]
pub struct AllFoundWordsProperties {
    pub cheat: bool,
    pub game_size: GameSize,
}

#[function_component(AllFoundWords)]
pub fn all_found_words(properties: &AllFoundWordsProperties) -> Html {
    let challenge_words = use_selector(|state: &FullGameState| state.game.challenge_words.clone());
    let found_words_state = use_selector(|state: &FullGameState| state.found_words.clone());
    let selected_tab_state = use_store_value::<SelectedTabState>();
    let selected_tab = selected_tab_state.index;

    let total_found = found_words_state.words.len();
    let cheat = properties.cheat;
    let game_size = properties.game_size;

    let words = (1..101)
        .map(|number| {
            let is_challenge = challenge_words.contains(&number);
            let is_found = found_words_state.words.contains_key(&number);
            html!(<FoundWordsWord {number} {is_challenge} {is_found} {selected_tab} {cheat} {game_size} />)
        })
        .collect::<Html>();

    html!(
        <div class="found-words">
            {words}

            <TodayGameButton {selected_tab} {game_size} width={6.0} position_number={101}/>
            <RandomGameButton {selected_tab} {game_size} width={6.0} position_number={111}/>
            <ScoreCounter {total_found} {selected_tab} {game_size} width={3.0} position_number={117}/>
            <FlipButton  {selected_tab} {game_size} width={1.0} position_number={109}/>
            <RotateButton  {selected_tab} {game_size} width={1.0} position_number={108}/>
            // <HistoryButton {selected_tab} {game_size} width={1.0} position_number={118}/>
            // <WainwrongButton {selected_tab} width={1.0} position_number={119}/>
            // <FacebookButton {selected_tab} width={1.0} position_number={116}/>

        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordProperties {
    pub number: i32,
    pub is_found: bool,
    pub is_challenge: bool,
    pub selected_tab: usize,
    pub cheat: bool,
    pub game_size: GameSize,
}

#[function_component(FoundWordsWord)]
pub fn found_words_word(properties: &FoundWordProperties) -> Html {
    //TODO onclick
    //TODO allow swiping to change tabs

    let id = format!("found_words_word{}", properties.number);
    let number = properties.number;
    let cheat = properties.cheat;

    let on_click: Option<Callback<MouseEvent>> = if properties.is_found || cheat {
        Some(Dispatch::new().apply_callback(move |_| FindNumberMsg { number, cheat }))
    } else {
        None
    };

    let color = if properties.is_found{
        "green"
    }else{
        "white"
    };

    // let rect_class = classes!(
    //     "found-word-box",
    //     if properties.is_found {
    //         Some("found-word-box-success")
    //     } else {
    //         None
    //     },
    //     if properties.is_challenge {
    //         Some("found-word-box-challenge")
    //     } else {
    //         None
    //     },
    // );
    // let text_class = classes!(
    //     "found-word-text",
    //     if properties.is_found {
    //         Some("found-word-text-success")
    //     } else {
    //         None
    //     }
    // );

    let text = format_number(number);

    //todo calculate position
    let (x, y) =
        properties
            .game_size
            .get_found_word_position(number, properties.selected_tab, false);

    html!(
        <FoundWordBox {id} {text} {x} {y} width_units={1.0} {color} {on_click} />
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordBoxProperties {
    pub id: String,
    pub text: AttrValue,
    pub color: AttrValue,
    pub x: f32,
    pub y: f32,
    pub width_units: f32,
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(FoundWordBox)]
pub fn found_word_box(properties: &FoundWordBoxProperties) -> Html {
    let x = properties.x;
    let y = properties.y;
    let width = format!("{}", FOUND_WORD_WIDTH * properties.width_units);
    let height = format!("{FOUND_WORD_HEIGHT}");
    let color = &properties.color;
    let style = format!("position:absolute; transform: translate({x}px, {y}px); height: {height}px; width: {width}px; border-radius:5px; background-color: {color};");

    let class = classes!(
        "found-word",
        if properties.on_click.is_some() {
            Some("found-word-button")
        } else {
            None
        }
    );
    let key = properties.id.clone();

    html!(
        <button {key} {style} {class} onclick={properties.on_click.clone()}>
            {properties.text.clone()}
        </button>
    )
}
