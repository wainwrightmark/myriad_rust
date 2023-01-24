use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::PointAbsolute8;
use myriad::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CirclesSVG)]
pub fn circles_svg() -> Html {
    let circles = PointAbsolute8::<GRID_COLUMNS, GRID_ROWS>::points_by_row()
        .map(|coordinate| html!(< Circle {coordinate} />))
        .collect::<Html>();

    html! {
          <g id="circles">
    {circles}
    </g>

      }
}

#[derive(PartialEq, Eq, Properties)]
pub struct CircleProperties {
    pub coordinate: PointAbsolute8<GRID_COLUMNS, GRID_ROWS>,
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let location = use_selector_with_deps(
        |state: &RotFlipState, co| state.get_location(co, SQUARE_SIZE),
        coordinate,
    );

    let board = use_selector(|state: &FullGameState| state.game.board.clone());

    let circle_type = *use_selector_with_deps(
        |state: &ChosenPositionsState, (co, board)| state.get_circle_type(co, &board),
        (coordinate, board),
    )
    .deref();

    let letter = *use_selector_with_deps(
        |state: &FullGameState, co| state.game.board[*co],
        coordinate,
    )
    .deref();

    let color = circle_type.get_color().to_string();
    let cursor = circle_type.get_cursor().to_string();

    let onpointerdown =
        Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Down { coordinate });

    let onpointerenter =
        Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Enter { coordinate });

    let cx = location.0;
    let cy = location.1;

    let text = if matches!(letter, Rune::Blank) {
        "".to_string()
    } else {
        letter.to_string()
    };
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

    html! {



        <g class="square"
        {key}
       style={g_style}
       cursor={cursor}

        {onpointerdown}
        {onpointerenter}
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
        dominant-baseline="central"
        text-anchor="middle"
        stroke="@Colors.Shades.White"
        fill="@Colors.Shades.Black">
        {text}
      </text>
    </g>
    }
}
