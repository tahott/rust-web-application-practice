use url::Url;
use yew::prelude::*;

use crate::{api::{FetchError, get_user, get_url_for_authorization_code, get_access_token}, types::{User, Career}};
use crate::components::CareerCard;

pub enum FetchState<T> {
  NotSignIn,
  NotFetching,
  Fetching,
  Success(T),
  Failed(FetchError),
}

pub enum Msg {
  None,
  GetUser,
  GetAuthorizationCode,
  GetJwt,
  SetUserFetchState(FetchState<User>),
  SetAccessTokenFetchState(FetchState<String>),
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
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let token = local_storage.get_item("token").unwrap().unwrap_or_default();

    if token != "" {
      ctx.link().send_message(Msg::GetUser);
      return Self {
        state: State { user: FetchState::NotFetching },
      }
    }

    Self {
      state: State { user: FetchState::NotSignIn },
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    let url = web_sys::window().unwrap().location().search().unwrap();
    if first_render == true && url.len() != 0 {
      ctx.link().send_message(Msg::GetJwt);
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::SetUserFetchState(fetch_state) => {
        self.state.user = fetch_state;
        true
      },
      Msg::SetAccessTokenFetchState(fetch_state) => {
        let token = match fetch_state {
          FetchState::NotSignIn => todo!(),
          FetchState::NotFetching => todo!(),
          FetchState::Fetching => todo!(),
          FetchState::Success(token) => token,
          FetchState::Failed(_) => todo!(),
        };

        let storage = web_sys::window().unwrap().local_storage().unwrap();
        storage.unwrap().set_item("token", &token).unwrap();

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
      },
      Msg::GetAuthorizationCode => {
        ctx.link().send_future(async move {
          match get_url_for_authorization_code().await {
            Ok(url) => {
              web_sys::window().unwrap().location().assign(url.as_str()).unwrap();
              Msg::None
            },
            Err(err) => Msg::SetUserFetchState(FetchState::Failed(err))
          }
        });
        true
      },
      Msg::GetJwt => {
        let url = web_sys::window().unwrap().location().search().unwrap();
        let temp = Url::parse(&(format!("http://localhost:3000/profile{}", url))).unwrap();
        let code_pair = temp
          .query_pairs()
          .find(|pair| {
            let &(ref key, _) = pair;
            key == "code"
          })
          .unwrap();

        let (_, value) = code_pair;
        let code = value.into_owned();

        ctx.link().send_future(async move {
          match get_access_token(code).await {
            Ok(token) => Msg::SetAccessTokenFetchState(FetchState::Success(token)),
            Err(err) => Msg::SetAccessTokenFetchState(FetchState::Failed(err)),
          }
        });

        // API로 access token 얻기
        // ctx.link().send_message(Msg::GetTrueAccessToken(code));
        true
      },
      Msg::None => true
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    match &self.state.user {
      FetchState::NotSignIn => html! {
        html! {
          <>
            <div class="grid place-items-center h-full">
              <div class="flex border-2 border-solid rounded p-4 cursor-pointer">
                <span class="iconify" data-icon="akar-icons:github-fill" style="font-size: 24px;"></span>
                <span class="pl-[16px] font-bold" onclick={ ctx.link().callback(|_| Msg::GetAuthorizationCode) }>        
                  {"깃허브 계정으로 로그인"}
                </span>
              </div>
            </div>
          </>
        }
      },
      FetchState::NotFetching => html! {},
      FetchState::Fetching => html! {},
      FetchState::Success(data) => {
        let careers: Vec<Html> = data.career.iter().map(|career: &Career| {
          html! {
            <CareerCard career={career.clone()} />
          }
        }).collect();

        html! {
          <div class="lg:pr-48 lg:pl-48">
            {data.name.clone()}
            <div class="rounded-b bg-gray-700 career_card_list flex flex-col m-2 mt-0 p-2 pt-0 pl-[16px] pr-[16px] md:grid grid-cols-12">{careers}</div>
          </div>
        }
      },
      FetchState::Failed(err) => html! { err },
    }
  }
}