use crate::core::prelude::*;
use crate::state::fullstate::*;
use crate::web::SQUARE_SIZE;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CirclesSVG)]
pub fn circles_svg() -> Html {
    let (game_state, _) = use_store::<FullState>();
    
    let circles = game_state
        .board
        .max_coordinate()
        .get_positions_up_to()
        .map(|c| make_circle(&game_state, c))
        .collect::<Html>();

    html! {
          <g>
    {circles}
    </g>

      }
}

fn make_circle(gamestate: &FullState, coordinate: Coordinate) -> Html {
    let location = gamestate.rotflip.get_location(&coordinate, SQUARE_SIZE);
    let cx = location.0;
    let cy = location.1;
    let color = gamestate.get_color(&coordinate).to_string();
    let letter = gamestate.board.get_letter_at_coordinate(&coordinate);
    let text = letter.word_text();
    let cursor = "default";
    let circle_id = format!("{coordinate}_bigCircle");
    let text_id = format!("{coordinate}_text");
    let radius = format!("{:2}", SQUARE_SIZE * 0.4);

    let g_style = format!(
        "-webkit-transform: translate({cx}px, {cy}px); transform: translate({cx}px, {cy}px);"
    );

    let onclick = Dispatch::new().apply_callback(move |_| Msg::Move { coordinate });

    html! {
        <g class="square"
       style={g_style}
       cursor={cursor}
       {onclick}
       >
      <circle
        id={circle_id}
        class="circle"
        stroke={color}
        fill="black"
        fill-opacity="0.01"
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
