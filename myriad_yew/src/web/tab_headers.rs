use crate::state::selected_tab_state::SelectedTabState;

use crate::state::prelude::*;
use crate::web::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TabHeaders)]
pub fn found_words_tab_headers() -> Html {
    let buttons = (1..=5)
        .map(|index| html!(<NumberTabHeader {index} /> ))
        .collect::<Html>();

    html!(<div class="tab-headers"> { buttons } <MoreTabHeader index={0} /> </div>)
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

    let (found, total) = *use_selector(|x: &FullGameState| x.get_found_count());

    let onclick = Dispatch::new().apply_callback(move |_| TabSelectedMsg { index });

    let key = format!("found_words_tab_header{index}");
    let selected = if selected_tab_state.index == index {
        Some("selected-tab")
    } else if selected_tab_state.locked {
        Some("locked-out-tab")
    } else {
        None
    };

    let background_color = if selected_tab_state.index == index {
        "var(--tab-background-selected)"
    } else if selected_tab_state.locked {
        "var(--tab-background-locked-out)"
    } else {
        "var(--tab-background-default)"
    };

    let (x, y) = game_size.get_tab_header_position(index);

    let found_pc = found * 100 / total;

    let class = classes!("tab-header", selected);
    let style = format!(
        "transform: translate({x}px, {y}px); background: linear-gradient(to right, var(--progress) {found_pc}%, {background_color} {found_pc}%, {background_color});",
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
        use_selector(move |state: &FullGameState| state.is_tab_complete((index as i32) - 1));

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
    let (x, y) = game_size.get_tab_header_position(index);

    let style = format!("transform: translate({x}px, {y}px);");

    html!(

        <button {class}  {style} {onclick} {key}>
        {format_number (((index as i32)) * GOALSIZE)}
     </button>

    )
}
