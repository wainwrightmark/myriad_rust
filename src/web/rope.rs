// use std::rc::Rc;

// use crate::state::chosenpositionsstate::*;
// use crate::state::fullstate::FullState;
// use crate::state::rotflipstate::RotFlipState;
// use crate::web::prelude::*;
// use itertools::Itertools;
// use num::ToPrimitive;
// use yew::prelude::*;
// use yewdux::prelude::*;

// #[function_component(RopeSVG)]
// pub fn rope_svg_g() -> Html {
//     let chosen_positions = use_selector(|state: &FullState| state.chosen_positions.clone());
//     let rot_flip = use_store_value::<RotFlipState>();

//     let opacity = if chosen_positions.positions.is_empty() {
//         "0"
//     } else {
//         "1"
//     };

//     let d = get_path_data( chosen_positions, rot_flip, SQUARE_SIZE);

//     let style = format!("stroke-width: {};", SVG_WIDTH / 20.0);

//     html! {
//                   <path
//     id="rope"
//     class="rope"
//     {style}
//     stroke-linejoin="round"
//     stroke-linecap="round"
//     {opacity}
//     {d}
//     />

//       }
// }

// fn get_path_data(
//     chosen_positions: Rc<ChosenPositionsState>,
//     rot_flip: Rc<RotFlipState>,
//     square_size: f64,
// ) -> String {
//     let coordinates = get_path_coordinates( chosen_positions, rot_flip, square_size);

//     let d = "M ".to_string()
//         + &coordinates
//             .iter()
//             .map(|x| format!("{:.2} {:.2}", x.0, x.1))
//             .join(" L ");

//     d
// }

// fn get_path_coordinates(    
//     chosen_positions: Rc<ChosenPositionsState>,
//     rot_flip: Rc<RotFlipState>,
//     square_size: f64,
// ) -> Vec<(f64, f64)> {
//     fn get_inbetween(d1: f64, d2: f64, numerator: f64, denominator: f64) -> f64 {
//         let t = d2 * numerator + d1 * (denominator - numerator);
//         t / denominator
//     }

//     if !chosen_positions.positions.is_empty() {
//         let locations = chosen_positions
//             .positions
//             .iter()
//             .map(|x| rot_flip.get_location(x, square_size))
//             .collect_vec();

//         //let total_letters = rot_flip.total_letters();
//         const ROPEPOINTS: usize = 60;

//         (0..ROPEPOINTS)
//             .map(|i| {
//                 let den = chosen_positions.positions.len();
//                 let index = (i * den) / ROPEPOINTS;
//                 let remainder = (i * den) % ROPEPOINTS;

//                 let loc = locations[index];

//                 if remainder == 0 || locations.len() <= index + 1 {
//                     loc
//                 } else {
//                     let next = locations[index + 1];

//                     (
//                         get_inbetween(loc.0, next.0, remainder as f64, ROPEPOINTS as f64),
//                         get_inbetween(loc.1, next.1, remainder as f64, ROPEPOINTS as f64),
//                     )
//                 }
//             })
//             .collect_vec()
//     } else {
//         let centre = (
//             square_size * rot_flip.columns().to_f64().unwrap() / 2.0,
//             square_size * rot_flip.rows().to_f64().unwrap() / 2.0,
//         );
//         let zero_vec = vec![centre; rot_flip.columns() as usize];
//         zero_vec
//     }
// }
