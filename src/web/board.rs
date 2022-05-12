use std::ops::Deref;

use crate::core::prelude::*;
use crate::state::fullstate::*;
use crate::web::prelude::*;
use crate::state::drag::DragMsg;
use yew::prelude::*;
use yewdux::prelude::*;


#[function_component(CirclesSVG)]
pub fn circles_svg() -> Html {
    let mc = use_selector(|state: &FullState| state.rotflip.max_coordinate.clone());

    let circles = mc
        .get_positions_up_to()
        .map(|coordinate| html!(<Circle {coordinate} />))
        .collect::<Html>();

    html! {
          <g>
    {circles}
    </g>

      }
}

#[derive(PartialEq, Properties)]
pub struct CircleProperties {
    coordinate: Coordinate,
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let location = use_selector_with_deps(
        |state: &FullState, co| state.rotflip.get_location(&co, SQUARE_SIZE),
        coordinate,
    );

    let (color, cursor) =
        use_selector_with_deps(|state: &FullState, co| state.get_color(&co), coordinate)
            .deref()
            .clone();

    let letter = use_selector_with_deps(
        |state: &FullState, co| state.board.get_letter_at_coordinate(&co),
        coordinate,
    )
    .deref()
    .clone();

    let ontouchend = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchEnd { coordinate });
    let onmousedown = Dispatch::new().apply_callback(move |_: MouseEvent| DragMsg::MouseDown { coordinate });

    let ontouchstart = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchStart { coordinate: coordinate });
    let onmouseup = Dispatch::new().apply_callback(move |_: MouseEvent| DragMsg::MouseUp { coordinate: coordinate });

    let cx = location.0;
    let cy = location.1;

    let text = letter.word_text();
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let radius = format!("{:2}", SQUARE_SIZE * 0.4);

    let g_style = format!(
        " -webkit-transform: translate({cx}px, {cy}px); transform: translate({cx}px, {cy}px);"
    );

    //let onclick = Dispatch::new().apply_callback(move |_| Msg::Move { coordinate });

    html! {
        <g class="square"
       style={g_style}
       cursor={cursor}
       //{onclick}
       {onmousedown}
       {onmouseup}
       {ontouchstart}
       {ontouchend}
       draggable="true"
       >
      <circle
        id={circle_id}
        class="circle"
        stroke={color}
        fill="black"
        fill-opacity="0.01"
        r={radius}
        >
      </circle>

      <text
        id={text_id}
        class="circle-text"
        dominant-baseline="middle"
        text-anchor="middle"
        stroke="@Colors.Shades.White"
        fill="@Colors.Shades.Black">
        {text}
      </text>
    </g>
    }
}
