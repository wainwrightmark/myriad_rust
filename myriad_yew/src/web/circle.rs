use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Circles)]
pub fn circles(game_size: &GameSize) -> Html {
    let game_size = *game_size;

    let circles = Tile::<GRID_COLUMNS, GRID_ROWS>::iter_by_row()
        .map(|coordinate| html!(< Circle {coordinate} {game_size} />))
        .collect::<Html>();

    let crosshairs = Tile::<GRID_COLUMNS, GRID_ROWS>::iter_by_row()
        .map(|coordinate| html!(< Crosshair {coordinate} {game_size} />))
        .collect::<Html>();

    let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    html! {
          <div id="circles" class="circles" {onpointerup}>
    {circles}
    {crosshairs}
    </div>

      }
}

#[derive(PartialEq, Properties)]
pub struct CircleProperties {
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    pub game_size: GameSize,
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let location = use_selector_with_deps(
        |state: &RotFlipState, (co, size)| state.get_location(co, *size),
        (coordinate, properties.game_size),
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

        let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    let square_radius = properties.game_size.square_radius();

    let left = location.x - (square_radius * CIRCLE_RATIO);
    let top = location.y - (square_radius * CIRCLE_RATIO);

    let text = if matches!(letter, Rune::Blank) {
        "".to_string()
    } else {
        letter.to_string()
    };
    let key = format!("{coordinate}_key");
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let diameter = format!("{:2}", square_radius * 2.0 * CIRCLE_RATIO);

    let g_style = format!("left: {left}px; top: {top}px;");

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
        {onpointerup}
        {onpointerenter}>

        <div
        id={circle_id}
        class={circle_classes}
        style={
            format!("width: {diameter}px;
            height: {diameter}px;
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
