use crate::state::prelude::*;
use myriad::prelude::{Tile, Center, HasCenter};

use num::ToPrimitive;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CrossHairProperties {
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
    pub game_size: GameSize,
}

// const CROSSHAIR_LENGTH: f32 = 15.0;
// const HALF_CROSSHAIR_LENGTH: f32 = CROSSHAIR_LENGTH / 2.0;
// const CROSSHAIR_INSET: f32 = 12.5;

const CROSSHAIR_LENGTH: f32 = 15.0;
const HALF_CROSSHAIR_LENGTH: f32 = CROSSHAIR_LENGTH / 2.0;
// const CROSSHAIR_INSET: f32 = 12.5;
const STROKE_WIDTH: f32 = 7.2;
const HALF_STOKE_WIDTH: f32 = 3.6;

#[function_component(Crosshair)]
pub fn crosshair(properties: &CrossHairProperties) -> Html {
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

    let location = rot_flip.get_location(&coordinate, properties.game_size);
    //let radius = properties.game_size.square_radius();


    let color = circle_type.get_color();

    let line_classes = match circle_type {
        CircleType::Disabled => "crosshair invisible",
        CircleType::LegalMove => "crosshair invisible",
        CircleType::LastPosition => "crosshair",
        CircleType::IntermediatePosition { next: _ } => "crosshair crosshair-extended",
    };
    let square_radius = properties.game_size.square_radius();
    let square_size = properties.game_size.square_length();

    const CROSSHAIR_INSET: f32 = 12.5;

    let l1 = match circle_type {
        CircleType::LastPosition => Transform {
            x: CROSSHAIR_INSET,
            y: square_radius,
            rot: 0.0,
        },
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 1, rot_flip, square_size)
        }
        _ => Transform {
            x: CROSSHAIR_INSET,
            y: square_radius,
            rot: 90.0,
        },
    };

    let l2 = match circle_type {
        CircleType::LastPosition => Transform {
            x: square_size - CROSSHAIR_INSET,
            y: square_radius,
            rot: 0.0,
        },
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 0, rot_flip, square_size)
        }
        _ => Transform {
            x: square_size - CROSSHAIR_INSET,
            y: square_radius,
            rot: 90.0,
        },
    };

    let l3 = match circle_type {
        CircleType::LastPosition => Transform {
            x: square_radius,
            y: CROSSHAIR_INSET,
            rot: -90.0,
        },
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 2, rot_flip, square_size)
        }
        _ => Transform {
            x: square_radius,
            y: CROSSHAIR_INSET,
            rot: 0.0,
        },
    };

    let l4 = match circle_type {
        CircleType::LastPosition => Transform {
            x: square_radius,
            y: square_size - CROSSHAIR_INSET,
            rot: -90.0,
        },
        CircleType::IntermediatePosition { next } => {
            get_line_position(coordinate, next, 3, rot_flip, square_size)
        }
        _ => Transform {
            x: square_radius,
            y: square_size - CROSSHAIR_INSET,
            rot: 0.0,
        },
    };

    let straight_scale_x: f32 = square_size / 4.0 / CROSSHAIR_LENGTH;
    let diagonal_scale_x: f32 = straight_scale_x * 1.42;

    let scale = if let CircleType::IntermediatePosition { next } = circle_type {
        if next.is_contiguous_with(&coordinate) {
            straight_scale_x
        } else {
            diagonal_scale_x
        }
    } else {
        1.0
    };
    // let left = location.x - radius;
    // let top = location.y - radius;

    // let left = -HALF_CROSSHAIR_LENGTH;
    // let top = -HALF_STOKE_WIDTH;
    let offset = Center{x:-square_radius, y: -square_radius};
    let style = (location + offset) .get_style();
    html!(
        <div key="crosshair" class={"crosshair-group"} {style}>

        <hr key="line1" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", l1.get_transform(scale) )} />
        <hr key="line2" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", l2.get_transform(scale) ) }/>

        <hr key="line3" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", l3.get_transform(scale) )} />
        <hr key="line4" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", l4.get_transform(scale) )} />
        </div>
    )
}

pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
}

impl Transform {
    pub fn get_transform(&self, scale: f32) -> String {
        format!(
            "transform: translate({}px, {}px) rotate({}deg) scaleX({});",
            self.x - (HALF_CROSSHAIR_LENGTH * 0.5), self.y -(HALF_STOKE_WIDTH * 0.5), self.rot, scale
        )
    }
}

fn get_line_position(
    c1: Tile<GRID_COLUMNS, GRID_ROWS>,
    c2: Tile<GRID_COLUMNS, GRID_ROWS>,
    index: u8,
    rf: RotFlipState,
    square_size: f32,
) -> Transform {
    let c1a = rotate_and_flip(&c1, rf.rotate, rf.flip);
    let c2a = rotate_and_flip(&c2, rf.rotate, rf.flip);

    let square_radius = square_size / 2.;

    let straight_scale_x: f32 = square_size / 4.0 / CROSSHAIR_LENGTH;
    let diagonal_scale_x: f32 = straight_scale_x * 1.42;

    let x_dir = c2a.col().to_f32().unwrap() - c1a.col().to_f32().unwrap();
    let y_dir = c2a.row().to_f32().unwrap() - c1a.row().to_f32().unwrap();

    let x = (x_dir * HALF_CROSSHAIR_LENGTH * diagonal_scale_x) - (x_dir * HALF_STOKE_WIDTH)
        + (x_dir / 4.0 * index.to_f32().unwrap() * square_size)
        + square_radius;
    let y = (y_dir * HALF_CROSSHAIR_LENGTH * diagonal_scale_x) - (y_dir * HALF_STOKE_WIDTH)
        + (y_dir / 4.0 * index.to_f32().unwrap() * square_size)
        + square_radius;

    let rot = f32::to_degrees(c1a.get_center(1.0).angle_to(&c2a.get_center(1.0)));

    Transform { x, y, rot }
}
