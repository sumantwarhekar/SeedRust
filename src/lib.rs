mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}