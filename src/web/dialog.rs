use crate::state::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Dialog)]
pub fn dialog() -> Html {
    let (state, dispatch) = use_store::<DialogState>();

    let onclick = dispatch.reduce_mut_callback(|state| state.dialog_type = None);

    if let Some(dialog_type) = state.dialog_type {
        let message: &str;
        //let quote: &str;

        match dialog_type {
            DialogType::Challenge => {
                message = "Well done, you beat challenge mode!\r\nNow try for 💯!";
                //quote = "I%20beat%20challenge%20mode%20in%20myriad%21";
            }
            DialogType::OneHundred => {
                message = "Well done, you got 💯!";
                //quote = "I%20got%20%F0%9F%92%AF%20in%20myriad%21"
            }
        }

        
        //let url = "https%3A%2F%2Fwainwrightmark.github.io%2Fmyriad_rust%2F";
        //let link = format!("https://www.facebook.com/sharer/sharer.php?u={}", url);

        let link = "https://www.facebook.com/sharer/sharer.php?u=wainwrightmark.github.io%2Fmyriad_rust";
        html!(<dialog style="top: 25%" open={true}>
        <p>{message}</p>
        <form>
      <button formaction={link}>{"Share!"}</button>
      <button {onclick}>{"Ok"}</button>
    
    </form>
      </dialog>)
    } else {
        return html!(<></>);
    }
}
