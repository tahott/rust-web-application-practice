use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  HomePage,
  #[at("/profile")]
  ProfilePage,
}