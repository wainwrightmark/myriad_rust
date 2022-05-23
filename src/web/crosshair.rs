use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;

use num::ToPrimitive;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CrosshairsSVG)]
pub fn crosshairs_svg() -> Html {
    

    let circles = Coordinate::get_positions_up_to::<GRID_COLUMNS, GRID_ROWS>()
        .map(|coordinate| html!(< Crosshair {coordinate} />))
        .collect::<Html>();

    html! {
          <g id="crosshairs">
    {circles}
    </g>

      }
}

#[derive(PartialEq, Properties)]
pub struct CrossHairProperties {
    coordinate: Coordinate,
}

const CROSSHAIR_LENGTH: f64 = 15.0;
const HALF_CROSSHAIR_LENGTH: f64 = CROSSHAIR_LENGTH / 2.0;
const CROSSHAIR_INSET: f64 = 12.5;

const STRAIGHT_SCALE_X: f64 = SQUARE_SIZE / 4.0 / CROSSHAIR_LENGTH;
const DIAGONAL_SCALE_X: f64 = STRAIGHT_SCALE_X * 1.42;

const HALF_STOKE_WIDTH: f64 = 3.6;

#[function_component(Crosshair)]
fn crosshair(properties: &CrossHairProperties) -> Html {
    let coordinate = properties.coordinate;
    let board = use_selector(|state: &FullState| state.board.clone())
        .deref()
        .clone();

    let circle_type = *use_selector_with_deps(
        |state: &ChosenPositionsState, (co, board)| state.get_circle_type(co, board.clone()),
        (coordinate, board),
    )
    .deref();

    let rot_flip = *use_store_value::<RotFlipState>().deref();

    let location = rot_flip.get_location(&coordinate, SQUARE_SIZE);

    let cx = location.0 - SQUARE_MIDPOINT;
    let cy = location.1 - SQUARE_MIDPOINT;

    let style = format!("stroke: {}; -webkit-transform: translate({cx}px, {cy}px); transform: translate({cx}px, {cy}px);", circle_type.get_color());
    let line_classes = match circle_type {
        CircleType::Disabled => "crosshair invisible",
        CircleType::LegalMove => "crosshair invisible",
        CircleType::LastPosition => "crosshair",
        CircleType::IntermediatePosition { next: _ } => "crosshair crosshair-extended",
    };

    let l1x = match circle_type {
        CircleType::LastPosition => CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 0, rot_flip).0
        }
        _ => CROSSHAIR_INSET,
    };

    let l1y = match circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 0, rot_flip).1
        }
        _ => SQUARE_MIDPOINT,
    };

    let l1rot = match circle_type {
        CircleType::LastPosition => 0.0,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 0, rot_flip).2
        }
        _ => 90.0,
    };

    let l2x = match circle_type {
        CircleType::LastPosition => SQUARE_SIZE - CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 1, rot_flip).0
        }
        _ => SQUARE_SIZE - CROSSHAIR_INSET,
    };

    let l2y = match circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 1, rot_flip).1
        }
        _ => SQUARE_MIDPOINT,
    };

    let l2rot = match circle_type {
        CircleType::LastPosition => 0.0,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 1, rot_flip).2
        }
        _ => 90.0,
    };

    let l3x = match circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 2, rot_flip).0
        }
        _ => SQUARE_MIDPOINT,
    };

    let l3y = match circle_type {
        CircleType::LastPosition => CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 2, rot_flip).1
        }
        _ => CROSSHAIR_INSET,
    };

    let l3rot = match circle_type {
        CircleType::LastPosition => -90.0,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 2, rot_flip).2
        }
        _ => 0.0,
    };

    let l4x = match circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 3, rot_flip).0
        }
        _ => SQUARE_MIDPOINT,
    };

    let l4y = match circle_type {
        CircleType::LastPosition => SQUARE_SIZE - CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 3, rot_flip).1
        }
        _ => SQUARE_SIZE - CROSSHAIR_INSET,
    };

    let l4rot = match circle_type {
        CircleType::LastPosition => -90.0,
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 3, rot_flip).2
        }
        _ => 0.0,
    };

    let scale = if let CircleType::IntermediatePosition { next } = circle_type {
        if next.is_orthogonal(coordinate) {
            STRAIGHT_SCALE_X
        } else {
            DIAGONAL_SCALE_X
        }
    } else {
        1.0
    };

    html!(
        <g key="crosshair" class={"crosshair-group"} {style}>

        <line key="line1" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={&(*line_classes)} style={format!("transform: translate({}px, {}px) rotate({}deg) scaleX({scale});", l1x, l1y, l1rot )} />
        <line key="line2" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={&(*line_classes)} style={format!("transform: translate({}px, {}px) rotate({}deg) scaleX({scale});", l2x, l2y, l2rot )}/>

        <line key="line3" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={&(*line_classes)} style={format!("transform: translate({}px, {}px) rotate({}deg) scaleX({scale});", l3x, l3y, l3rot )} />
        <line key="line4" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({}px, {}px) rotate({}deg) scaleX({scale});", l4x, l4y, l4rot )} />
        </g>
    )
}

fn get_line_position(
    c1: Coordinate,
    c2: Coordinate,
    index: u8,
    rf: RotFlipState,
) -> (f64, f64, f64) {
    let c1a = c1.rotate_and_flip::<GRID_COLUMNS, GRID_ROWS> ( rf.rotate, rf.flip);
    let c2a = c2.rotate_and_flip::<GRID_COLUMNS, GRID_ROWS>( rf.rotate, rf.flip);

    let x_dir = c2a.column.to_f64().unwrap() - c1a.column.to_f64().unwrap();
    let y_dir = c2a.row.to_f64().unwrap() - c1a.row.to_f64().unwrap();

    let x = (x_dir * HALF_CROSSHAIR_LENGTH * DIAGONAL_SCALE_X) - (x_dir * HALF_STOKE_WIDTH)
        + (x_dir / 4.0 * index.to_f64().unwrap() * SQUARE_SIZE)
        + SQUARE_MIDPOINT;
    let y = (y_dir * HALF_CROSSHAIR_LENGTH * DIAGONAL_SCALE_X) - (y_dir * HALF_STOKE_WIDTH)
        + (y_dir / 4.0 * index.to_f64().unwrap() * SQUARE_SIZE)
        + SQUARE_MIDPOINT;

    let rot = c1a.get_angle(c2a);

    (x, y, rot)
}
