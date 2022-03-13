use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum Route {
  #[at("/")]
  HomePage,
  #[at("/career")]
  CareerPage,
  #[at("/profile")]
  ProfilePage,
}