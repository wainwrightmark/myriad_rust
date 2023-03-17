use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::{Tile, HasCenter};

use num::ToPrimitive;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CrosshairsSVG)]
pub fn crosshairs_svg() -> Html {
    let circles = Tile::<GRID_COLUMNS, GRID_ROWS>::iter_by_row()
        .map(|coordinate| html!(< Crosshair {coordinate} />))
        .collect::<Html>();

    html! {
          <g id="crosshairs">
    {circles}
    </g>

      }
}

#[derive(PartialEq, Eq, Properties)]
pub struct CrossHairProperties {
    coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
}

const CROSSHAIR_LENGTH: f32 = 15.0;
const HALF_CROSSHAIR_LENGTH: f32 = CROSSHAIR_LENGTH / 2.0;
const CROSSHAIR_INSET: f32 = 12.5;

const STRAIGHT_SCALE_X: f32 = SQUARE_SIZE / 4.0 / CROSSHAIR_LENGTH;
const DIAGONAL_SCALE_X: f32 = STRAIGHT_SCALE_X * 1.42;

const HALF_STOKE_WIDTH: f32 = 3.6;

#[function_component(Crosshair)]
fn crosshair(properties: &CrossHairProperties) -> Html {
    let coordinate = properties.coordinate;
    let board = use_selector(|state: &FullGameState| state.game.board.clone())
        .deref()
        .clone();

    let circle_type = *use_selector_with_deps(
        |state: &ChosenPositionsState, (co, board)| state.get_circle_type(co, board),
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
        if next.is_contiguous_with(&coordinate) {
            STRAIGHT_SCALE_X
        } else {
            DIAGONAL_SCALE_X
        }
    } else {
        1.0
    };

    html!(
        <g key="crosshair" class={"crosshair-group"} {style}>

        <line key="line1" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({l1x}px, {l1y}px) rotate({l1rot}deg) scaleX({scale});" )} />
        <line key="line2" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({l2x}px, {l2y}px) rotate({l2rot}deg) scaleX({scale});" )}/>

        <line key="line3" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({l3x}px, {l3y}px) rotate({l3rot}deg) scaleX({scale});" )} />
        <line key="line4" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({l4x}px, {l4y}px) rotate({l4rot}deg) scaleX({scale});" )} />
        </g>
    )
}

fn get_line_position(
    c1: Tile<GRID_COLUMNS, GRID_ROWS>,
    c2: Tile<GRID_COLUMNS, GRID_ROWS>,
    index: u8,
    rf: RotFlipState,
) -> (f32, f32, f32) {
    let c1a = rotate_and_flip(&c1, rf.rotate, rf.flip).get_vertex(&myriad::prelude::Corner::NorthEast).unwrap() .get_center(1.0);
    let c2a = rotate_and_flip(&c2, rf.rotate, rf.flip).get_vertex(&myriad::prelude::Corner::NorthEast).unwrap().get_center(1.0);

    let x_dir = c2a.x - c1a.x;
    let y_dir = c2a.y - c1a.y;

    let x = (x_dir * HALF_CROSSHAIR_LENGTH * DIAGONAL_SCALE_X) - (x_dir * HALF_STOKE_WIDTH)
        + (x_dir / 4.0 * index.to_f32().unwrap() * SQUARE_SIZE)
        + SQUARE_MIDPOINT;
    let y = (y_dir * HALF_CROSSHAIR_LENGTH * DIAGONAL_SCALE_X) - (y_dir * HALF_STOKE_WIDTH)
        + (y_dir / 4.0 * index.to_f32().unwrap() * SQUARE_SIZE)
        + SQUARE_MIDPOINT;

    let rot = f32::to_degrees(c1a.angle_to(&c2a));

    (x, y, rot)
}
