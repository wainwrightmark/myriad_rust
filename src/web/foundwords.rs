use crate::state::{fullstate::*, GOALSIZE};
use crate::web::newgamebutton::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FoundWordsTableContent)]
pub fn found_words_table_content() -> Html {
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let tab_content = (0..5)
        .map(|twenties| {
            let chips = (1..GOALSIZE + 1)
                .map(|units| {
                    let i = twenties * GOALSIZE + units;

                    let found = found_words_state.words.contains_key(&i);
                    let class = if found {"label success chip"} else {"label chip"};

                    html!(<span key={format!("chip{i}")} {class}>{i}</span>)
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
pub fn found_words_table() -> Html {
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let checked_tab = found_words_state.most_recent.map_or(0, |x| (x - 1) / GOALSIZE);

    let tab_labels = (0..(100 / GOALSIZE))
        .map(|group_index| {

            let complete = found_words_state.has_all_words(&mut num::iter::range( (group_index * GOALSIZE) + 1, ((group_index + 1) * GOALSIZE) + 1));

            let style = if complete{Some("background-color: #2ecc40;")} else{None};
            let id = format!("tab-{group_index}");
            let label = format!("{:0>2}", (group_index * GOALSIZE) + 1);
            

            html! {
                <key={id} >
                <input  id={id.clone()} type="radio" name="tabgroupB" checked={group_index == checked_tab} />
                <label class="pseudo button toggle" for={id.clone()} {style}>{label}</label>
                </>
            }
        })
        .collect::<Html>();

    let tab_count = match (100 / GOALSIZE) + 1 {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        _ => "seven",
    };

    html! {
        <div>
        <div class={format!("tabs {tab_count}")}> // should depend on number of groups
        {tab_labels}
        <input id="plus_tab" type="radio" name="tabgroupB" />
        <label class="pseudo button toggle" for="plus_tab" >{"➕"}</label>

        <div class="row">
        <FoundWordsTableContent/>
        <div>
        <NewGameButton/>
        </div>

        </div>
        </div>
        </div>
    }
}
