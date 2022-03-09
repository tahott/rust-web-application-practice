use yew::prelude::*;

use crate::{api::{FetchError, get_user}, types::User};

pub enum FetchState<T> {
  NotFetching,
  Fetching,
  Success(T),
  Failed(FetchError),
}

pub enum Msg {
  GetUser,
  SetUserFetchState(FetchState<User>),
}

struct State {
  user: FetchState<User>,
}

pub struct Profile {
  state: State,
}

impl Component for Profile {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    ctx.link().send_message(Msg::GetUser);
    Self {
      state: State { user: FetchState::NotFetching },
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SetUserFetchState(fetch_state) => {
        self.state.user = fetch_state;
        true
      },
      Msg::GetUser => {
        ctx.link().send_future(async {
          match get_user().await {
            Ok(user) => Msg::SetUserFetchState(FetchState::Success(user)),
            Err(err) => Msg::SetUserFetchState(FetchState::Failed(err))
          }
        });
        ctx.link().send_message(Msg::SetUserFetchState(FetchState::Fetching));
        true
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    match &self.state.user {
      FetchState::NotFetching => html! {},
      FetchState::Fetching => html! {},
      FetchState::Success(data) => {
        html! {
          <div class="lg:pr-48 lg:pl-48">
            {data.nick.clone()}
          </div>
        }
      },
      FetchState::Failed(err) => html! { err },
    }
    // html! {
    //   "This page is profile"
    // }
  }
}