mod app;
mod route;
mod pages;
mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  yew::start_app::<app::App>();  
}