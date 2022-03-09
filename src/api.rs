use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use rand::{thread_rng, Rng};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request, Response};

use crate::types::User;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
  err: JsValue,
}

impl Display for FetchError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Debug::fmt(&self.err, f)
  }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
  fn from(value: JsValue) -> Self {
    Self { err: value }
  }
}

pub async fn get_user() -> Result<User, FetchError> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init("../products/users.json", &opts)?;

  let window = gloo_utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  let users: Vec<User> = json.into_serde().unwrap();

  let mut rng = thread_rng();

  Ok(users[rng.gen_range(0..users.len())].clone())
}