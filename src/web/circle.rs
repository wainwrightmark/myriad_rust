use std::ops::Deref;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;

use num::ToPrimitive;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CrossHairProperties {
    circle_type: CircleType,
    coordinate: Coordinate
}

pub const CROSSHAIR_LENGTH: f64 =  15.0;
//pub const EXTENDED_CROSSHAIR_SCALE : f64 = (SQUARE_SIZE / 5.0 * 1.42) / CROSSHAIR_LENGTH; //1.42 = Root Two
pub const HALF_CROSSHAIR_LENGTH: f64 = CROSSHAIR_LENGTH / 2.0;
pub const CROSSHAIR_INSET: f64 = 12.5;
pub const EXTENDED_SCALE: f64 = 2.27; //(SQUARE_SIZE / 5.0 * root 2) / CROSSHAIR_LENGTH;

fn get_line_position(c1 : Coordinate, c2 : Coordinate, index: u8, rf: RotFlipState) -> (f64,  f64,  f64){

    let c1a = c1.rotate_and_flip(rf.max_coordinate, rf.rotate, rf.flip);
    let c2a = c2.rotate_and_flip(rf.max_coordinate, rf.rotate, rf.flip);

    let x_dir =c2a.column.to_f64().unwrap() - c1a.column.to_f64().unwrap();
    let y_dir = c2a.row.to_f64().unwrap() - c1a.row.to_f64().unwrap();


    let x = (x_dir * HALF_CROSSHAIR_LENGTH  * EXTENDED_SCALE) + (x_dir / 4.0 * index.to_f64().unwrap() * SQUARE_SIZE) + SQUARE_MIDPOINT;
    let y = (y_dir * HALF_CROSSHAIR_LENGTH * EXTENDED_SCALE) + (y_dir / 4.0 * index.to_f64().unwrap() * SQUARE_SIZE) + SQUARE_MIDPOINT;

    let rot = c1a.get_angle(c2a);

    (x,y,rot)
}

#[function_component(Crosshair)]
pub fn crosshairs(properties: &CrossHairProperties) -> Html {

    let rot_flip_state = use_store_value::<RotFlipState>();
    let rot_flip = RotFlipState{rotate: rot_flip_state.rotate, flip: rot_flip_state.flip, max_coordinate: rot_flip_state.max_coordinate, };

    let style = format!("stroke: {};", properties.circle_type.get_color());
    let line_classes = match properties.circle_type {
        CircleType::Disabled => "crosshair invisible",
        CircleType::LegalMove => "crosshair invisible",
        CircleType::LastPosition => "crosshair",
        CircleType::IntermediatePosition { next: _ } => "crosshair crosshair-extended",
    };

    let l1x = match properties.circle_type {
        CircleType::LastPosition => CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 0, rot_flip).0,
        _ => CROSSHAIR_INSET,
    };

    let l1y = match properties.circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } =>  get_line_position(properties.coordinate, next, 0, rot_flip).1,
        _ => SQUARE_MIDPOINT,
    };

    let l1rot = match properties.circle_type {
        CircleType::LastPosition => 0.0,
        CircleType::IntermediatePosition { next } =>  get_line_position(properties.coordinate, next, 0, rot_flip).2,
        _ => 90.0,
    };

    let l2x = match properties.circle_type {
        CircleType::LastPosition => SQUARE_SIZE - CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } =>  get_line_position(properties.coordinate, next, 1, rot_flip).0,
        _ => SQUARE_SIZE - CROSSHAIR_INSET,
    };

    let l2y = match properties.circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next} => get_line_position(properties.coordinate, next, 1, rot_flip).1,
        _ => SQUARE_MIDPOINT,
    };

    let l2rot = match properties.circle_type {
        CircleType::LastPosition => 0.0,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 1, rot_flip).2,
        _ => 90.0,
    };

    let l3x = match properties.circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 2, rot_flip).0,
        _ => SQUARE_MIDPOINT,
    };

    let l3y = match properties.circle_type {
        CircleType::LastPosition => CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 2, rot_flip).1,
        _ => CROSSHAIR_INSET,
    };

    let l3rot = match properties.circle_type {
        CircleType::LastPosition => -90.0,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 2, rot_flip).2,
        _ => 0.0,
    };

    let l4x = match properties.circle_type {
        CircleType::LastPosition => SQUARE_MIDPOINT,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 3, rot_flip).0,
        _ => SQUARE_MIDPOINT,
    };

    let l4y = match properties.circle_type {
        CircleType::LastPosition => SQUARE_SIZE - CROSSHAIR_INSET,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next, 3, rot_flip).1,
        _ => SQUARE_SIZE - CROSSHAIR_INSET,
    };

    let l4rot = match properties.circle_type {
        CircleType::LastPosition => -90.0,
        CircleType::IntermediatePosition { next } => get_line_position(properties.coordinate, next,3, rot_flip).2,
        _ => 0.0,
    };

    let scale = if matches!(properties.circle_type, CircleType::IntermediatePosition{next:_}) {2.27} else{1.0};

    html!(
        <g key="crosshair" class={"crosshair-group"} {style}>

        <line key="line1" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes.clone()} style={format!("transform: translate({}px, {}px) rotate({}deg) scale({scale});", l1x, l1y, l1rot )} />
        <line key="line2" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes.clone()} style={format!("transform: translate({}px, {}px) rotate({}deg) scale({scale});", l2x, l2y, l2rot )}/>

        <line key="line3" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes.clone()} style={format!("transform: translate({}px, {}px) rotate({}deg) scale({scale});", l3x, l3y, l3rot )} />
        <line key="line4" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_classes} style={format!("transform: translate({}px, {}px) rotate({}deg) scale({scale});", l4x, l4y, l4rot )} />
        </g>
    )
}

#[derive(PartialEq, Properties)]
pub struct CircleProperties {
    pub coordinate: Coordinate,
}
#[function_component(Circle)]
pub fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let location = use_selector_with_deps(
        |state: &RotFlipState, co| state.get_location(&co, SQUARE_SIZE),
        coordinate,
    );

    let circle_type = use_selector_with_deps(
        |state: &FullState, co| state.get_circle_type(&co),
        coordinate,
    )
    .deref()
    .clone();

    let letter = use_selector_with_deps(
        |state: &FullState, co| state.board.get_letter_at_coordinate(&co),
        coordinate,
    )
    .deref()
    .clone();

    let color = circle_type.get_color().to_string();
    let cursor = circle_type.get_cursor().to_string();

    //let ontouchend = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchEnd { coordinate });
    //let onmousedown = Dispatch::new().apply_callback(move |_: MouseEvent| DragMsg::MouseDown { coordinate });

    //let ontouchstart = Dispatch::new().apply_callback(move |_: TouchEvent| DragMsg::TouchStart { coordinate: coordinate });
    //let onmouseup = Dispatch::new().apply_callback(move |_: MouseEvent| DragMsg::MouseUp { coordinate: coordinate });

    let onclick = Dispatch::new().apply_callback(move |_: MouseEvent| Msg::Move {
        coordinate: coordinate,
    });

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
       {onclick}
       //{onmousedown}
       //{onmouseup}
       //{ontouchstart}
       //{ontouchend}
       draggable="true"
       >
      <circle
        id={circle_id}
        class={circle_classes}
        stroke={color}
        r={radius}
        >
      </circle>

      <Crosshair {circle_type} {coordinate} />

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
