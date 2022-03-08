use yew::prelude::*;

pub struct Profile {}

impl Component for Profile {
  type Message = ();
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      "This page is profile"
    }
  }
}