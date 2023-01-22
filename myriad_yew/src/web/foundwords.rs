use std::rc::Rc;

use crate::state::foundwordsstate::FoundWordsState;
use crate::state::selectedtabstate::SelectedTabState;

use crate::state::prelude::*;
use crate::web::prelude::*;
use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FoundWordsTabHeaders)]
pub fn found_words_tab_headers() -> Html {
    let state = use_selector(|state: &FullGameState| state.found_words.clone());
    let selected_tab_state = use_store_value::<SelectedTabState>();

    let buttons = (0..5)
        .map(|index| found_words_tab_header(index, state.clone(), selected_tab_state.clone()))
        .collect::<Html>();

    html!(<g> { buttons } <MoveTabHeader index={5} {selected_tab_state}/> </g>)
}

#[derive(PartialEq, Eq, Properties)]
pub struct MoreTabHeaderProperties {
    index: usize,
    selected_tab_state: Rc<SelectedTabState>,
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

    let class = classes!("tab-header", selected);
    let style = format!(
        "transform: translate({}px, {}px);",
        TAB_HEADER_PADDING + (index.to_f64().unwrap() * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN)),
        (SQUARE_SIZE * 3.0) + TAB_HEADER_TOP_MARGIN
    );

    html!(
        <g {key} {style} {onclick}>
        <rect {class} height={format!("{TAB_HEADER_HEIGHT}")} rx="5" width={format!("{TAB_HEADER_WIDTH}")}>
        </rect>
        <text class="tab-header-text" fill="Black" stroke="White">
           {"☰"}
        </text>

        </g>
    )
}

pub fn found_words_tab_header(
    index: usize,
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
        TAB_HEADER_PADDING + (index.to_f64().unwrap() * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN)),
        (SQUARE_SIZE * 3.0) + TAB_HEADER_TOP_MARGIN
    );
    html!(
    <g {key} {style} {onclick}>
    <rect {class} height={format!("{TAB_HEADER_HEIGHT}")} rx="5" width={format!("{TAB_HEADER_WIDTH}")}>
    </rect>
    <text class="tab-header-text" fill="Black" stroke="White">
       {format_number ((index.to_i32().unwrap()  + 1) * GOALSIZE)}
    </text>

    </g>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct AllFoundWordsProperties {
    pub cheat: bool,
}

#[function_component(AllFoundWords)]
pub fn all_found_words(properties: &AllFoundWordsProperties) -> Html {
    let challenge_words = use_selector(|state: &FullGameState| state.game.challenge_words.clone());
    let found_words_state = use_selector(|state: &FullGameState| state.found_words.clone());
    let selected_tab_state = use_store_value::<SelectedTabState>();
    let selected_tab = selected_tab_state.index;

    let total_found = found_words_state.words.len();
    let cheat = properties.cheat;

    let words = (1..101)
        .map(|number| {
            let is_challenge = challenge_words.contains(&number);
            let is_found = found_words_state.words.contains_key(&number);
            html!(<FoundWordsWord {number} {is_challenge} {is_found} {selected_tab} {cheat} />)
        })
        .collect::<Html>();

    html!(
        <g>
            {words}

            <TodayGameButton {selected_tab} width={6.0} position_number={101}/>
            <RandomGameButton {selected_tab} width={6.0} position_number={111}/>
            <ScoreCounter {total_found} {selected_tab} width={1.5} position_number={106}/>
            <FlipButton  {selected_tab} width={1.0} position_number={109}/>
            <RotateButton  {selected_tab} width={1.0} position_number={108}/>
            <HistoryButton {selected_tab} width={1.0} position_number={118}/>
            <WainwrongButton {selected_tab} width={1.0} position_number={119}/>
            <FacebookButton {selected_tab} width={1.0} position_number={116}/>

        </g>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct FoundWordProperties {
    pub number: i32,
    pub is_found: bool,
    pub is_challenge: bool,
    pub selected_tab: usize,
    pub cheat: bool,
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

    let rect_class = classes!(
        "found-word-box",
        if properties.is_found {
            Some("found-word-box-success")
        } else {
            None
        },
        if properties.is_challenge {
            Some("found-word-box-challenge")
        } else {
            None
        },
    );
    let text_class = classes!(
        "found-word-text",
        if properties.is_found {
            Some("found-word-text-success")
        } else {
            None
        }
    );

    let text = format_number(number);

    //todo calculate position
    let (x, y) = get_found_word_position(number, properties.selected_tab, false);

    html!(
        <FoundWordBox {id} {text} {x} {y} width_units={1.0} {rect_class} {text_class} {on_click} />
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordBoxProperties {
    pub id: String,
    pub text: String,
    pub rect_class: Classes,
    pub text_class: Classes,
    pub x: f64,
    pub y: f64,
    pub width_units: f64,
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(FoundWordBox)]
pub fn found_word_box(properties: &FoundWordBoxProperties) -> Html {
    let x = properties.x;
    let y = properties.y;
    let style = format!("transform: translate({x}px, {y}px);");

    let class = classes!(
        "found-word-group",
        if properties.on_click.is_some() {
            Some("found-word-group-button")
        } else {
            None
        }
    );
    let role = if properties.on_click.is_some() {
        Some("button")
    } else {
        None
    };

    html!(
     <g key={properties.id.clone()} {style}  {role} {class} onclick={properties.on_click.clone()}>
     <rect class={properties.rect_class.clone()} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width_units)}>
     </rect>
     <text class={properties.text_class.clone()}>
        {properties.text.clone()}
     </text>

     </g>
    )
}

pub fn get_found_word_position(number: i32, selected_index: usize, clamp: bool) -> (f64, f64) {
    let row_number = ((number - 1) % GOALSIZE) / 10;
    let y = BOARD_HEIGHT
        + TAB_HEADER_HEIGHT
        + TAB_HEADER_TOP_MARGIN
        + FOUND_WORD_MARGIN
        + (FOUND_WORD_HEIGHT + FOUND_WORD_MARGIN) * row_number.to_f64().unwrap();

    let row_position = ((number - 1) % GOALSIZE) % 10;

    let tab_x = FOUND_WORD_PADDING
        + row_position.to_f64().unwrap() * (FOUND_WORD_MARGIN + FOUND_WORD_WIDTH);

    let index = (number - 1) / GOALSIZE;
    let mut index_offset = (index - selected_index.to_i32().unwrap()).to_f64().unwrap();
    if clamp {
        index_offset = index_offset.min(1.0).max(-1.0);
    }

    let offset_x = index_offset * SVG_WIDTH;

    let x = tab_x + offset_x;
    (x, y)
}
