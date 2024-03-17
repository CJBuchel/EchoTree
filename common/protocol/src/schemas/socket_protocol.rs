use schemars::JsonSchema;

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct Checksum {
  pub tree: String, // name of tree
  pub checksum: u32, // checksum value
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct MethodParameters {
  pub trees: Option<Vec<String>>, // vector of tree names
  pub key: Option<String>, // key name
  pub checksums: Option<Vec<Checksum>>, // checksum values of a tree
  pub data: Option<String>, // data to be stored (generic, usually json endpoint of a tree)
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub enum MethodType{
  // generic methods
  Set, // tree, key, data -> to server
  Get, // tree, key -> to server
  Delete, // tree, key -> to server
  Checksum, // checksums -> to server (server should send Echo if any of the checksums mismatch)

  // echo sync methods
  Echo, // checksum, data <- to client
  Subscribe, // tree -> to server
  Unsubscribe, // tree -> to server
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoEvent {
  pub auth_token: String,
  pub method: MethodType,
  pub params: MethodParameters,
}