mod app;
mod api;
mod route;
mod pages;
mod types;
mod components;

pub fn main() {
  yew::Renderer::<app::App>::new().render();
}