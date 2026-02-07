mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}