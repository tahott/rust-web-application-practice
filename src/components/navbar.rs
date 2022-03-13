use yew::prelude::*;

use crate::route::Route;
use crate::components::navbar_item::NavItem;

pub struct NavBar {}

impl Component for NavBar {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <nav class="block border-b border-inherit grid grid-cols-4 h-[64px] m-0 p-0 pl-[16px] pr-[16px] md:grid-cols-8 lg:grid-cols-12 content-center gap-x-[20px] lg:pl-[24px] lg:pr-[24px]">
        <div class="h-[64px] col-span-4 md:col-span-8 lg:col-span-12 flex flex-row justify-between items-center lg:pr-48 lg:pl-48">
          <NavItem
            route={Route::HomePage}
            icon="ant-design:home-outlined"
            name="HOME"
          />
          <NavItem
            route={Route::CareerPage}
            icon="raphael:roadmap"
            name="Career"
          />
          <NavItem
            route={Route::ProfilePage}
            icon="ant-design:user-outlined"
            name="My Profile"
          />
        </div>
      </nav>
    }
  }
}