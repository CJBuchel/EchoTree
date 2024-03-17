use schemars::JsonSchema;

pub mod socket_protocol;
pub mod http_protocol;

/**
 * Role used for authentication to branches of the database
 */
#[derive(serde::Deserialize, serde::Serialize, Clone, JsonSchema)]
pub struct Role {
  pub role_id: String,
  pub password: String,
  pub echo_trees: Vec<String>, // list of topics/trees the role can access
}