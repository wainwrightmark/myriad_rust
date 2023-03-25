use crate::state::prelude::*;
use myriad::prelude::*;

use num::ToPrimitive;
use std::ops::{Add, Deref};
use yew::prelude::*;
use yewdux::prelude::*;
#[function_component(Crosshairs)]
pub fn crosshairs() -> Html {
    let crosshairs = Tile::<GRID_COLUMNS, GRID_ROWS>::iter_by_row()
        .map(|coordinate| html!(< Crosshair {coordinate}  />))
        .collect::<Html>();

    html!(<div class="crosshairs">{crosshairs}</div>)
}

#[derive(PartialEq, Properties)]
pub struct CrossHairProperties {
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
}

const CROSSHAIR_LENGTH: f32 = 20.0;
const HALF_CROSSHAIR_LENGTH: f32 = CROSSHAIR_LENGTH / 2.0;
// const CROSSHAIR_INSET: f32 = 12.5;
const STROKE_WIDTH: f32 = 7.2;
const HALF_STOKE_WIDTH: f32 = 3.6;

#[function_component(Crosshair)]
pub fn crosshair(properties: &CrossHairProperties) -> Html {
    let (game_size, _) = use_store::<GameSize>();
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

    let location = rot_flip.get_location(&coordinate, game_size.as_ref());
    //let radius = properties.game_size.square_radius();

    let color = circle_type.get_color();

    let line_classes = match circle_type {
        CircleType::Disabled => "crosshair invisible",
        CircleType::LegalMove => "crosshair invisible",
        CircleType::LastPosition => "crosshair",
        CircleType::IntermediatePosition { next: _ } => "crosshair crosshair-extended",
    };
    let square_radius = game_size.square_radius();
    let square_size = game_size.square_length();

    let l1 = Transform::get_section_transform(
        circle_type,
        square_radius,
        square_size,
        coordinate,
        1,
        rot_flip,
    );
    let l2 = Transform::get_section_transform(
        circle_type,
        square_radius,
        square_size,
        coordinate,
        2,
        rot_flip,
    );
    let l3 = Transform::get_section_transform(
        circle_type,
        square_radius,
        square_size,
        coordinate,
        3,
        rot_flip,
    );
    let l4 = Transform::get_section_transform(
        circle_type,
        square_radius,
        square_size,
        coordinate,
        4,
        rot_flip,
    );

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
    let offset = Center {
        x: -(square_radius * 0.9),
        y: -(square_radius * 0.9),
    };
    // let style = (location + offset).get_style();

    let top_left = location + offset;
    html!(
        <div key="crosshair" class={"crosshair-group"} >

        <hr key="line1" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", (l1 + top_left).get_transform(scale) )} />
        <hr key="line2" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", (l2 + top_left).get_transform(scale) ) }/>

        <hr key="line3" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", (l3 + top_left).get_transform(scale) )} />
        <hr key="line4" class={line_classes} style={format!("background-color: {color};  width: {CROSSHAIR_LENGTH}px; height:{STROKE_WIDTH}px; {}", (l4 + top_left).get_transform(scale) )} />
        </div>
    )
}

pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
}

impl Add<Center> for Transform {
    type Output = Self;

    fn add(self, rhs: Center) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            rot: self.rot,
        }
    }
}

impl Transform {
    pub fn get_section_transform(
        circle_type: CircleType,
        square_radius: f32,
        square_size: f32,

        coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
        index: u8,
        rot_flip: RotFlipState,
    ) -> Self {
        const CROSSHAIR_INSET: f32 = 10.;

        match circle_type {
            CircleType::IntermediatePosition { next } => {
                let new_index = match index {
                    1 => 1,
                    2 => 0,
                    3 => 2,
                    4 => 3,
                    _ => unreachable!(),
                };
                get_line_position(coordinate, next, new_index, rot_flip, square_size)
            }

            _ => {
                let rot = match (index, circle_type) {
                    (1 | 2, CircleType::LastPosition) => 0.0,
                    (1 | 2, _) => 90.0,
                    (3 | 4, CircleType::LastPosition) => -90.0,
                    (3 | 4, _) => 0.0,
                    _ => unreachable!(),
                };

                let x = match index {
                    1 => CROSSHAIR_INSET,
                    2 => square_size - CROSSHAIR_INSET,
                    3 => square_radius,
                    4 => square_radius,
                    _ => unreachable!(),
                };

                let y = match index {
                    1 => square_radius,
                    2 => square_radius,
                    3 => CROSSHAIR_INSET,
                    4 => square_size - CROSSHAIR_INSET,
                    _ => unreachable!(),
                };

                Self { x, y, rot }
            }
        }
    }

    pub fn get_transform(&self, scale: f32) -> String {
        format!(
            "transform: translate({}px, {}px) rotate({}deg) scaleX({});",
            self.x - (HALF_CROSSHAIR_LENGTH),
            self.y - (HALF_STOKE_WIDTH),
            self.rot,
            scale
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

    let straight_scale_x: f32 = square_size / 4.5 / CROSSHAIR_LENGTH;
    let diagonal_scale_x: f32 = straight_scale_x * 1.43;

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
