use std::rc::Rc;

use crate::state::foundwordsstate::FoundWordsState;
use crate::state::selectedtabstate::SelectedTabState;

use crate::state::{fullstate::*, msg::*, GOALSIZE};
use crate::web::*;
use crate::web::prelude::*;
use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FoundWordsTabHeaders)]
pub fn found_words_tab_headers() -> Html {
    let state = use_selector(|state: &FullState| state.found_words.clone());
    let selected_tab_state = use_selector(|state: &FullState| state.selected_tab_state);

    let buttons = (0..5)
        .map(|index| found_words_tab_header(index, state.clone(), selected_tab_state.clone()))
        .collect::<Html>();

    html!(<g> { buttons } <MoveTabHeader index={5} {selected_tab_state}/> </g>)
}

#[derive(PartialEq, Properties)]
pub struct MoreTabHeaderProperties {
    index: usize,
    selected_tab_state: Rc<SelectedTabState>,
}

#[function_component(MoveTabHeader)]
pub fn more_tab_header(properties: &MoreTabHeaderProperties) -> Html {
    let index = properties.index;
    let selected_tab_state = &properties.selected_tab_state;

    let onclick = Dispatch::new().apply_callback(move |_| Msg::SelectTab { index });

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
        index.to_f64().unwrap() * TAB_HEADER_WIDTH,
        SQUARE_SIZE * 3.0
    );

    html!(
        <g {key} {style} {onclick}>
        <rect {class} height={format!("{TAB_HEADER_HEIGHT}")} rx="5" width={format!("{TAB_HEADER_WIDTH}")}>
        </rect>
        <text class="tab-header-text" fill="Black" stroke="White">
           {"+"}
        </text>

        </g>
    )
}

pub fn found_words_tab_header(
    index: usize,
    state: Rc<Rc<FoundWordsState>>,
    selected_tab_state: Rc<SelectedTabState>,
) -> Html {
    let onclick = Dispatch::new().apply_callback(move |_| Msg::SelectTab { index });

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
        index.to_f64().unwrap() * TAB_HEADER_WIDTH,
        SQUARE_SIZE * 3.0
    );
    html!(
    <g {key} {style} {onclick}>
    <rect {class} height={format!("{TAB_HEADER_HEIGHT}")} rx="5" width={format!("{TAB_HEADER_WIDTH}")}>
    </rect>
    <text class="tab-header-text" fill="Black" stroke="White">
       {format!("{:0>2}", (index.to_i32().unwrap() * GOALSIZE) + 1)}
    </text>

    </g>
    )
}

#[function_component(AllFoundWords)]
pub fn all_found_words() -> Html {
    let state = use_selector(|state: &FullState| state.found_words.clone());
    let selected_tab_state = use_selector(|state: &FullState| state.selected_tab_state);
    let selected_tab = selected_tab_state.index;

    let total_found = state.words.len();

    let words = (1..101)
        .map(|number| {
            let is_found = state.words.contains_key(&number);
            html!(<FoundWordsWord {number} {is_found} {selected_tab} />)
        })
        .collect::<Html>();

    html!(
        <g>
            {words}

            <NewGameButton {selected_tab}/>
            <ScoreCounter {total_found} {selected_tab}/>
            <FlipButton  {selected_tab}/>
            <RotateButton  {selected_tab}/>

        </g>
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordProperties {
    pub number: i32,
    pub is_found: bool,
    pub selected_tab: usize,
}



#[function_component(FoundWordsWord)]
pub fn found_words_word(properties: &FoundWordProperties) -> Html {
    //TODO onclick
    //TODO allow swiping to change tabs

    let id = format!("found_words_word{}", properties.number);
    let number = properties.number;

    let success = if properties.is_found {
        Some("found-word-box-success")
    } else {
        None
    };

    let on_click: Option<Callback<MouseEvent>> = if properties.is_found {
        Some(Dispatch::new().apply_callback(move |_| Msg::Find {
            number,
        }))
    } else {
        None
    };

    let rect_class = classes!("found-word-box", success);

    let text = if number == 100 {
        "💯".to_string()
    } else {
        format!("{:0>2}", number)
    };

    //todo calculate position
    let (x, y) = get_found_word_position(number, properties.selected_tab, false);

    html!(
        <FoundWordBox {id} {text} {x} {y} width_units={1.0} rect_class= {rect_class} {on_click} />
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordBoxProperties {
    pub id: String,
    pub text: String,
    pub rect_class: Classes,
    pub x: f64,
    pub y: f64,
    pub width_units: f64,
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(FoundWordBox)]
pub fn found_word_box(properties: &FoundWordBoxProperties) -> Html {
    let x = properties.x;
    let y = properties.y;
    let style = format!("transform: translate({}px, {}px);", x, y);

    html!(
     <g key={properties.id.clone()} {style} class="found-word-group" role="button" onclick={properties.on_click.clone()}>
     <rect class={properties.rect_class.clone()} height={format!("{FOUND_WORD_HEIGHT}")} rx="5" width={format!("{}", FOUND_WORD_WIDTH * properties.width_units)}>
     </rect>
     <text class="found-word-text" fill="white" stroke="white">
        {properties.text.clone()}
     </text>

     </g>
    )
}

pub fn get_found_word_position(number: i32, selected_index: usize, clamp: bool) -> (f64, f64) {
    let row_number = ((number - 1) % GOALSIZE) / 10;
    let y = BOARD_HEIGHT
        + TAB_HEADER_HEIGHT
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
