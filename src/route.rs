use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  HomePage,
  #[at("/career")]
  CareerPage,
  #[at("/profile")]
  ProfilePage,
}
