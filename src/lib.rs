mod app;
mod api;
mod route;
mod pages;
mod types;
mod components;

pub fn run_app() {
  yew::Renderer::<app::App>::new().render();
}