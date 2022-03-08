mod app;
mod route;
mod pages;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<app::App>();  
}