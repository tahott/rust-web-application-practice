use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, Profile};
use crate::route::Route;
use crate::components::NavBar;

pub struct App {}

fn switch(routes: &Route) -> Html {
  match routes.clone() {
    Route::HomePage => {
      html! {
        <Home />
      }
    },
    Route::ProfilePage => {
      html! {
        <Profile />
      }
    },
  }
}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div class="bg-[#1f2937] text-slate-50 min-w-[360px] h-screen md:min-w-[768px] lg:min-w-[1024px]" id="body">
        <BrowserRouter>
          <NavBar />
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
  }
}