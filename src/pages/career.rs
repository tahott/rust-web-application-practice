use yew::prelude::*;

pub struct Career;

impl Component for Career {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
      <div>
        {"This page is Career"}
      </div>
    }
  }
}