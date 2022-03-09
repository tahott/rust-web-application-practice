use yew::prelude::*;
use yew_router::components::Link;

use crate::route::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NavItemProps {
  pub route: Route,
  pub icon: String,
  pub name: String,
}

#[function_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {
  html! {
    <Link<Route> to={props.route}>
      <span class="iconify inline"
      data-icon={props.icon.clone()} style="font-size: 24px; color: white;"></span>
      <span class="pl-[8px] hidden lg:inline">{&props.name}</span>
    </Link<Route>>
  }
}