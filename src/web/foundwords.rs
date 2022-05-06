use crate::state::fullstate::*;
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
