use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::PointAbsolute8;
use myriad::prelude::*;

use num::Zero;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Circles)]
pub fn circles() -> Html {

    let node =  use_node_ref();
    let (width, height) = yew_hooks::use_size(node.clone());
    let mut size = (width.min(height) as f32) / 3.;
    //log::info!("{size}");
    if size.is_zero() {size = SQUARE_SIZE};
    let circles = PointAbsolute8::<GRID_COLUMNS, GRID_ROWS>::points_by_row()
        .map(|coordinate| html!(< Circle {coordinate} {size} />))
        .collect::<Html>();

        let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    html! {
          <div ref={node} id="circles" style="position:absolute; width: 100%; aspect-ratio: 1/1;" {onpointerup}>
    {circles}
    </div>

      }
}

#[derive(PartialEq,  Properties)]
pub struct CircleProperties {
    pub coordinate: PointAbsolute8<GRID_COLUMNS, GRID_ROWS>,
    pub size: f32
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;
    let size = properties.size;

    let location = use_selector_with_deps(
        |state: &RotFlipState, (co, size)| state.get_location(co, *size),
        (coordinate, size),
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

    let cx = location.0 - (size * 0.4);
    let cy = location.1 - (size * 0.4);

    let text = if matches!(letter, Rune::Blank) {
        "".to_string()
    } else {
        letter.to_string()
    };
    let key = format!("{coordinate}_key");
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let radius = format!("{:2}", size * 0.4);
    let diameter = format!("{:2}", size * 0.8);

    let g_style = format!(
        "left: {cx}px; top: {cy}px;"
    );

    let circle_type_class = match circle_type {
        CircleType::Disabled => "circle-disabled",
        CircleType::LegalMove => "circle-legal",
        CircleType::LastPosition => "circle-final",
        CircleType::IntermediatePosition { next: _ } => "circle-intermediate",
    };

    let circle_classes = classes!("circle", circle_type_class);

    html! {

        <div class="square"
        {key}
       style={g_style}
       cursor={cursor}

        {onpointerdown}
        {onpointerenter}>

        <div
        id={circle_id}
        class={circle_classes}
        style={
            format!("width: {diameter}px;
            height: {diameter}px;
            border-radius: {radius}px;
            border-color: {color};")
        }
        >
        <p
        id={text_id}
        class="circle-text">
        {text}
      </p>
      </div>



        </div>
    }
}
