use std::ops::Deref;

use crate::state::prelude::*;
use crate::web::prelude::*;
use myriad::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Circles)]
pub fn circles() -> Html {
    let circles = Tile::<GRID_COLUMNS, GRID_ROWS>::iter_by_row()
        .map(|coordinate| html!(< Circle {coordinate} />))
        .collect::<Html>();

    let ontouchmove = Dispatch::new().apply_callback(move |ev: TouchEvent| {
        if let Some(tile) = get_tile_from_touch_event(ev) {
            InputMsg::Enter { coordinate: tile }
        } else {
            InputMsg::None
        }
    });

    let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    html! {
      <div id="circles" class="circles" {onpointerup} {ontouchmove}>
          {circles}
      </div>

    }
}

fn get_tile_from_touch_event(ev: TouchEvent) -> Option<Tile<GRID_COLUMNS, GRID_ROWS>> {
    let touch = ev.target_touches().item(0)?;

    let window = window()?;
    let document = window.document()?;
    let x = touch.page_x();
    let y = touch.page_y();
    let element = document.element_from_point(x as f32, y as f32)?;

    let id = element.id();

    //log::info!("touch move to {id}");

    TILES.get(id.as_str()).cloned()
}

use phf::phf_map;

static TILES: phf::Map<&'static str, Tile<GRID_COLUMNS, GRID_ROWS>> = phf_map! {
    "(0,0)_bigCircle" => Tile::new_const::<0,0>(),
    "(0,1)_bigCircle" => Tile::new_const::<0,1>(),
    "(0,2)_bigCircle" => Tile::new_const::<0,2>(),
    "(1,0)_bigCircle" => Tile::new_const::<1,0>(),
    "(1,1)_bigCircle" => Tile::new_const::<1,1>(),
    "(1,2)_bigCircle" => Tile::new_const::<1,2>(),
    "(2,0)_bigCircle" => Tile::new_const::<2,0>(),
    "(2,1)_bigCircle" => Tile::new_const::<2,1>(),
    "(2,2)_bigCircle" => Tile::new_const::<2,2>(),
};

#[derive(PartialEq, Properties)]
pub struct CircleProperties {
    pub coordinate: Tile<GRID_COLUMNS, GRID_ROWS>,
}
#[function_component(Circle)]
fn circle(properties: &CircleProperties) -> Html {
    let coordinate = properties.coordinate;

    let game_size = *use_store::<GameSize>().0.as_ref();
    let location = use_selector_with_deps(
        |state: &RotFlipState, (co, size)| state.get_location(co, size),
        (coordinate, game_size),
    );

    let board = use_selector(|state: &FullGameState| state.game.board.clone());

    let circle_type = *use_selector_with_deps(
        |state: &ChosenPositionsState, (co, board)| state.get_circle_type(co, board),
        (coordinate, board),
    )
    .deref();

    let letter = *use_selector_with_deps(
        |state: &FullGameState, co| state.game.board[*co],
        coordinate,
    )
    .deref();

    let onpointerdown = Dispatch::new().apply_callback(move |ev: PointerEvent| {
        ev.target()
            .unwrap()
            .dyn_into::<Element>()
            .unwrap()
            .release_pointer_capture(ev.pointer_id())
            .unwrap();
        InputMsg::Down { coordinate }
    });

    let onpointerenter =
        Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Enter { coordinate });

    let onpointerup = Dispatch::new().apply_callback(move |_: PointerEvent| InputMsg::Up {});

    let square_radius = game_size.square_radius();

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

    let g_style = format!("left: {left}px; top: {top}px;");

    let circle_type_class = match circle_type {
        CircleType::Disabled => "circle-disabled",
        CircleType::LegalMove => "circle-legal",
        CircleType::LastPosition => "circle-final",
        CircleType::IntermediatePosition { next: _ } => "circle-intermediate",
    };

    let circle_classes = classes!("circle", circle_type_class);

    html! {
        <div class="square" {key} style={g_style}>

            <div id={circle_id} class={circle_classes} {onpointerdown} {onpointerup} {onpointerenter}>
                <p id={text_id} class="circle-text"> {text} </p>
            </div>
        </div>
    }
}
