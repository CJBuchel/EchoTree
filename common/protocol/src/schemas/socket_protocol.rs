use schemars::JsonSchema;

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct MethodParameters {
  pub trees: Option<Vec<String>>,
  pub key: Option<String>,
  pub data: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub enum MethodType{
  // generic methods
  Set, // tree, key, data
  Get, // tree, key
  Delete, // tree, key

  // echo sync methods
  Echo, // tree, data
  Subscribe, // tree
  Unsubscribe, // tree
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoEvent {
  pub auth_token: String,
  pub method: MethodType,
  pub params: MethodParameters,
}