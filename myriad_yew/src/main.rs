#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use crate::web::prelude::*;
pub mod state;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}