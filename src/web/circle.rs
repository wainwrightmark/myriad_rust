use std::ops::Deref;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CrossHairProperties {
    circle_type: CircleType,
}

pub const CROSSHAIR_LENGTH: f64 = 15.0;
pub const HALF_CROSSHAIR_LENGTH: f64 = CROSSHAIR_LENGTH / 2.0;
pub const CROSSHAIR_INSET: f64 = 12.5;


#[function_component(Crosshair)]
pub fn crosshairs(properties: &CrossHairProperties) -> Html {
    let style = format!("stroke: {};", properties.circle_type.get_color());
    let class = "crosshair-group";

    

    let line_class =  if properties.circle_type == CircleType::LastPosition{ 
        classes!("crosshair") 
    }else{
        classes!("crosshair", "invisible") 
    };

    let line1_style = if properties.circle_type == CircleType::LastPosition{
        format!("transform: translate({}px, {}px) rotate(0deg);",CROSSHAIR_INSET, SQUARE_MIDPOINT)
    }else{
        format!("transform: translate({}px, {}px) rotate(90deg);", CROSSHAIR_INSET, SQUARE_MIDPOINT) 
    };

    let line2_style = if properties.circle_type == CircleType::LastPosition{
        format!("transform: translate({}px, {}px) rotate(0deg);",  SQUARE_SIZE - CROSSHAIR_INSET , SQUARE_MIDPOINT)
    }else{
        format!("transform: translate({}px, {}px) rotate(90deg);",SQUARE_SIZE - CROSSHAIR_INSET, SQUARE_MIDPOINT) 
    };
    
    
    let line3_style = if properties.circle_type == CircleType::LastPosition{
        format!("transform: translate({}px, {}px) rotate(-90deg);", SQUARE_MIDPOINT,  CROSSHAIR_INSET , )
    }else{
        format!("transform: translate({}px, {}px) rotate(0deg);", SQUARE_MIDPOINT, CROSSHAIR_INSET) 
    };

    let line4_style = if properties.circle_type == CircleType::LastPosition{
        format!("transform: translate({}px, {}px) rotate(-90deg);",SQUARE_MIDPOINT,  SQUARE_SIZE - CROSSHAIR_INSET )
    }else{
        format!("transform: translate({}px, {}px) rotate(0deg);",SQUARE_MIDPOINT, SQUARE_SIZE - CROSSHAIR_INSET) 
    };


    html!(
        <g key="crosshair" class={class} {style}>

        <line key="line1" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_class.clone()} style={line1_style} />
        <line key="line2" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_class.clone()} style={line2_style}/>

        <line key="line3" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_class.clone()} style={line3_style} />
        <line key="line4" x1={(-HALF_CROSSHAIR_LENGTH).to_string()} x2={HALF_CROSSHAIR_LENGTH.to_string()} y1={0.0.to_string()} y2={0.0.to_string()}  class={line_class} style={line4_style} />
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

      <Crosshair {circle_type} />

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
