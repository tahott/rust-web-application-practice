use std::{
  error::Error,
  fmt::{self, Debug, Display, Formatter},
};
use rand::{thread_rng, Rng};
use reqwasm::http::{Request, Method};
use serde::{Serialize, Deserialize};
use url::Url;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, RequestMode, Request as R, Response};

use crate::types::User;

#[derive(Serialize)]
struct CodeBody {
  provider: String,
  auth_code: String,
}

#[derive(Serialize, Deserialize)]
struct Res {
  data: String
}

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

  let request = R::new_with_str_and_init("../products/users.json", &opts)?;

  let window = gloo_utils::window();
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Response = resp_value.dyn_into().unwrap();

  let json = JsFuture::from(resp.json()?).await?;
  let users = serde_wasm_bindgen::from_value::<Vec<User>>(json).unwrap();

  let mut rng = thread_rng();

  Ok(users[rng.gen_range(0..users.len())].clone())
}

pub async fn get_url_for_authorization_code() -> Result<Url, FetchError> {
  let url = Request::get("http://localhost:8082/authorization/code")
    .send()
    .await.unwrap().json::<Url>().await.unwrap();

  Ok(url)
}

pub async fn get_access_token(code: String) -> Result<String, FetchError> {
  let json = &CodeBody {
    provider: "Github".to_string(),
    auth_code: code,
  };

  let resp = Request::new("http://localhost:8082/signin")
    .method(Method::POST)
    .mode(RequestMode::Cors)
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(json).unwrap())
    .send()
    .await
    .unwrap();

  let json = resp.json::<Res>().await.unwrap();

  Ok(json.data)
}