use std::ops::Deref;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CirclesSVG)]
pub fn circles_svg() -> Html {
    let circles = Coordinate::get_positions_up_to::<GRID_COLUMNS, GRID_ROWS>()
        .map(|coordinate| html!(< Circle {coordinate} />))
        .collect::<Html>();

    html! {
          <g id="circles">
    {circles}
    </g>

      }
}

#[derive(PartialEq, Properties)]
pub struct CircleProperties {
    pub coordinate: Coordinate,
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let location = use_selector_with_deps(
        |state: &RotFlipState, co| state.get_location(co, SQUARE_SIZE),
        coordinate,
    );

    let board = use_selector(|state: &FullState| state.board.clone())
        .deref()
        .clone();

    let circle_type = *use_selector_with_deps(
        |state: &ChosenPositionsState, (co, board)| state.get_circle_type(co, board.clone()),
        (coordinate, board),
    )
    .deref();

    let letter = *use_selector_with_deps(
        |state: &FullState, co| state.board.get_letter_at_coordinate(co),
        coordinate,
    )
    .deref();

    let color = circle_type.get_color().to_string();
    let cursor = circle_type.get_cursor().to_string();

    //let ontouchend = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchEnd { coordinate });
    let onpointerdown =
        Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Down { coordinate });

    let onpointerenter =
        Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Enter { coordinate });

    //let ontouchstart = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchStart { coordinate: coordinate });

    // let onclick = Dispatch::new().apply_callback(move |_: MouseEvent| OnClickMsg {
    //     coordinate,
    //     allow_abandon: true
    // });

    let cx = location.0;
    let cy = location.1;

    let text = letter.word_text();
    let key = format!("{coordinate}_key");
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let radius = format!("{:2}", SQUARE_SIZE * 0.4);

    let g_style = format!(
        " -webkit-transform: translate({cx}px, {cy}px); transform: translate({cx}px, {cy}px);"
    );

    let circle_type_class = match circle_type {
        CircleType::Disabled => "circle-disabled",
        CircleType::LegalMove => "circle-legal",
        CircleType::LastPosition => "circle-final",
        CircleType::IntermediatePosition { next: _ } => "circle-intermediate",
    };

    let circle_classes = classes!("circle", circle_type_class);

    //let onclick = Dispatch::new().apply_callback(move |_| Msg::Move { coordinate });

    html! {



        <g class="square"
        {key}
       style={g_style}
       cursor={cursor}

        {onpointerdown}
        //{onpointerup}
        {onpointerenter}


       //{onclick}
       //{onmousedown}
       //{onmouseup}
       //{onmouseenter}

    //    {ontouchstart}
    //    {ontouchend}
    //    {ontouchmove}
       >


      <circle
        id={circle_id}
        class={circle_classes}
        stroke={color}
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
