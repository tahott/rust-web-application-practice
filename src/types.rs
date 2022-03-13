use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
  pub id: u8,
  pub name: String,
  pub career: Vec<Career>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Career {
  pub name: String,
  pub in_date: String,
  pub out_date: String,
  pub job: String,
}