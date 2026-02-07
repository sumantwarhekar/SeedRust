mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}