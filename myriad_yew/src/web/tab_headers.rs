use crate::state::selected_tab_state::SelectedTabState;

use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TabHeaders)]
pub fn found_words_tab_headers() -> Html {
    let buttons = (0..5)
        .map(|index| html!(<NumberTabHeader {index} /> ))
        .collect::<Html>();

    html!(<div class="tab-headers"> { buttons } <MoreTabHeader index={5} /> </div>)
}

#[derive(PartialEq, Properties)]
pub struct MoreTabHeaderProperties {
    index: usize,
}

#[function_component(MoreTabHeader)]
pub fn more_tab_header(properties: &MoreTabHeaderProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let index = properties.index;
    let selected_tab_state = use_store_value::<SelectedTabState>();

    let onclick = Dispatch::new().apply_callback(move |_| TabSelectedMsg { index });

    let key = format!("found_words_tab_header{index}");
    let selected = if selected_tab_state.index == index {
        Some("selected-tab")
    } else if selected_tab_state.locked {
        Some("locked-out-tab")
    } else {
        None
    };

    let x = game_size.get_tab_header_padding()
        + (index as f32 * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN));
    let y = (game_size.square_length() * 3.0) + TAB_HEADER_TOP_MARGIN;

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

#[derive(PartialEq, Properties)]
pub struct NumberTabHeaderProperties {
    index: usize,
}

#[function_component(NumberTabHeader)]
pub fn found_words_tab_header(properties: &NumberTabHeaderProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
    let index = properties.index;
    let onclick = Dispatch::new().apply_callback(move |_| TabSelectedMsg { index });
    let selected_tab_state = use_store_value::<SelectedTabState>();
    let is_complete =
        use_selector(move |state: &FullGameState| state.found_words.is_goal_complete(index));

    let key = format!("found_words_tab_header{index}");
    let selected = if selected_tab_state.index == index {
        Some("selected-tab")
    } else if selected_tab_state.locked {
        Some("locked-out-tab")
    } else {
        None
    };
    let complete = is_complete.then(|| "complete-tab");

    let class = classes!("tab-header", selected, complete);
    let style = format!(
        "transform: translate({}px, {}px);",
        game_size.get_tab_header_padding()
            + (index as f32 * (TAB_HEADER_WIDTH + TAB_HEADER_MARGIN)),
        (game_size.square_length() * 3.0) + TAB_HEADER_TOP_MARGIN
    );

    html!(

        <button {class}  {style} {onclick} {key}>
        {format_number ((index as i32  + 1) * GOALSIZE)}
     </button>

    )
}
