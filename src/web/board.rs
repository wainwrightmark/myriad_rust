use crate::state::prelude::*;
use crate::web::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;


#[function_component(CirclesSVG)]
pub fn circles_svg() -> Html {
    let mc = use_selector(|state: &FullState| state.board.max_coordinate());

    let circles = mc
        .get_positions_up_to()
        .map(|coordinate| html!(< Circle {coordinate} />))
        .collect::<Html>();

    html! {
          <g>
    {circles}
    </g>

      }
}

