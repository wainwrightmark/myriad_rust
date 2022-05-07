use crate::state::{fullstate::*, GOALSIZE};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(FoundWordsTableContent)]
pub fn found_words_table_content() -> Html {
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let tab_content = (0..5)
        .map(|twenties| {
            let chips = (0..20)
                .map(|units| {
                    let i = twenties * 20 + units;

                    if i == 0 {
                        html!(<span class="label chip" style="visibility: hidden;"></span>)
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
pub fn found_words_table() -> Html {
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let checked_tab = found_words_state.most_recent.map_or(0, |x| x / GOALSIZE);

    let tab_labels = (0..(100 / GOALSIZE))
        .map(|group_index| {

            let complete = found_words_state.has_all_words(&mut num::iter::range( (group_index * GOALSIZE).max(1), (group_index + 1) * GOALSIZE));

            let style = if complete{"background-color: #2ecc40;"} else{""};
            let id = format!("tab-{group_index}");
            let label = format!("{:0>2}", (group_index * GOALSIZE));
            

            html! {
                <>
                <input id={id.clone()} type="radio" name="tabgroupB" checked={group_index == checked_tab} />
                <label class="pseudo button toggle" for={id} style={style}>{label}</label>
                </>
            }
        })
        .collect::<Html>();

    let tab_count = match 100 / GOALSIZE {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        _ => "seven"
    };

    html! {
        <div>
        <div class={format!("tabs {tab_count}")}> // should depend on number of groups
        {tab_labels}
        <div class="row">
        <FoundWordsTableContent/>
        </div>
        </div>
        </div>
    }
}
