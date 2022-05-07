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
    let found_words_state = use_selector(|state: &FullState| state.found_words.clone());

    let tab_labels = (0..5)
        .map(|twenties| {

            let complete = found_words_state.has_all_words(&mut num::iter::range( (twenties * 20).max(1), (twenties + 1) * 20));

            let style = if complete{"background-color: #2ecc40;"} else{""};
            let id = format!("tab-{twenties}");
            let label = format!("{:0>2}", (twenties * 20));
            html! {
                <>
                <input id={id.clone()} type="radio" name="tabgroupB" checked={twenties == 0} />
                <label class="pseudo button toggle" for={id} style={style}>{label}</label>
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
