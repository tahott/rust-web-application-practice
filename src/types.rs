use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
  pub id: u8,
  pub nick: String,
  pub company: String,
}