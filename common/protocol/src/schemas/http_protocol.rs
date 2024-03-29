use std::collections::HashMap;

use schemars::JsonSchema;


#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct RegisterRequest {
  pub echo_trees: Vec<String>, // list of topics/trees the client is subscribed to
  pub role_id: Option<String>, // optional role id for the client
  pub password: Option<String>, // optional password for the client
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct RegisterResponse {
  pub uuid: String,
  pub url: String,
  pub auth_token: String,
  pub hierarchy: HashMap<String, String>, // tree, schema
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct RoleAuthenticateRequest {
  pub role_id: String,
  pub password: String,
}