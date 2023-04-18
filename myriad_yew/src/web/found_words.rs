use crate::state::info_bar_state::InfoBarSetMessage;
use crate::state::selected_tab_state::SelectedTabState;

use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AllFoundWordsProperties {
    pub cheat: bool,
}

#[function_component(AllFoundWords)]
pub fn all_found_words(properties: &AllFoundWordsProperties) -> Html {
    let found_words = use_selector(|state: &FullGameState| state.found_words.clone());
    let difficulties = use_selector(|state: &FullGameState| state.game.difficulties.clone());
    let selected_tab_state = use_store_value::<SelectedTabState>();
    let selected_tab = selected_tab_state.index;

    let cheat = properties.cheat;

    let words = (1i32..=100i32)
        .filter_map(|number| {
            difficulties
                .get((number - 1) as usize)
                .and_then(|&x| x)
                .map(|x| (number, x))
        })
        .map(|(number, difficulty)| {
            let is_found = found_words.words.contains_key(&number);
            html!(<FoundWordsWord {number}  {is_found} {selected_tab} {cheat} {difficulty}  />)
        })
        .collect::<Html>();

    html!(
        <div class="found-words">
            {words}

            <TodayGameButton {selected_tab}  width={6.0} position_number={101}/>

            <DarkModeButton  {selected_tab} width={1.0} position_number={107}/>
            <RotateButton  {selected_tab} width={1.0} position_number={108}/>
            <FlipButton  {selected_tab}  width={1.0} position_number={109}/>
            <ShareButton {selected_tab} width={1.0} position_number={110} />

            <RandomGameButton {selected_tab}  width={6.0} position_number={111}/>
            <ScoreCounter {selected_tab}  width={3.0} position_number={117}/>




            // <HistoryButton {selected_tab} {game_size} width={1.0} position_number={118}/>

        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct FoundWordProperties {
    pub number: i32,
    pub difficulty: Difficulty,
    pub is_found: bool,
    pub selected_tab: usize,
    pub cheat: bool,
}

#[function_component(FoundWordsWord)]
pub fn found_words_word(properties: &FoundWordProperties) -> Html {
    //TODO allow swiping to change tabs

    let key = format!("found_words_word{}", properties.number);
    let number = properties.number;
    let cheat = properties.cheat;
    let difficulty = properties.difficulty;

    let on_click: Option<Callback<MouseEvent>> = if properties.is_found || cheat {
        Some(Dispatch::new().apply_callback(move |_| FindNumberMsg { number, cheat }))
    } else {
        Some(Dispatch::new().apply_callback(move |_| {
            InfoBarSetMessage(crate::state::info_bar_state::InfoBarState::Difficulty(
                difficulty,
            ))
        }))
    };

    let text = format_number(number);
    let (game_size, _) = use_store::<GameSize>();
    let (x, y) = game_size.get_found_word_position(number, properties.selected_tab, false);

    let style = format!(" transform: translate({x}px, {y}px); height: {FOUND_WORD_HEIGHT}px; width: {FOUND_WORD_WIDTH}px;");

    let class = classes!(
        "found-word",
        "found-word-button",
        "found-number",
        properties.is_found.then_some(Some("found-word-complete"))
    );
    html!(
        <button {key} {style} {class} onclick={on_click}>
            {text.clone()}

        </button>
    )
}
