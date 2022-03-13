use yew::prelude::*;

use crate::types::Career;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub career: Career,
}

pub struct CareerCard;

impl Component for CareerCard {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="career_card_container flex md:contents">
        <div class="col-start-1 col-end-2 mr-10 md:mx-auto relative">
          <div class="h-full w-4 flex items-center justify-center">
            <div class="h-full w-1 bg-gray-500 pointer-events-none"></div>
          </div>
          <div class="w-4 h-4 absolute top-1/2 -mt-3 rounded-full bg-green-500 shadow text-center">
            <i class="fas fa-check-circle text-white"></i>
          </div>
        </div>
        <div class="career_card col-start-2 col-end-12 pt-4">
          <div>{&ctx.props().career.name}</div>
          <div>{&ctx.props().career.job}</div>
          <div>{&ctx.props().career.in_date}{" ~ "}{&ctx.props().career.out_date}</div>
        </div>
      </div>
    }
  } 
}