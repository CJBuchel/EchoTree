use schemars::JsonSchema;

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct HashTree {
  pub tree: String, // name of tree
  pub checksum: Option<u32>, // checksum value (optional)
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub enum EchoMethodType{
  // echo sync methods (server -> client)
  EchoTree, // trees, data <- to client
  EchoItem, // trees, key, data <- to client
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoEvent { // echo event is from server to client
  pub method: EchoMethodType,
  pub trees: Option<Vec<HashTree>>, // checksum values of a tree
  pub key: Option<String>, // key name
  pub data: Option<String>, // data to be stored (generic, usually json endpoint of a tree)
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub enum OperationMethodType {
  // generic methods
  Set, // trees, key, data -> to server
  Get, // trees, key -> to server
  Delete, // trees, key -> to server
  Checksum, // trees -> to server (server should send Echo if any of the checksums mismatch)

  // subscription methods
  Subscribe, // tree -> to server
  Unsubscribe, // tree -> to server
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct OperationRequest { // operation request is from client to server
  pub auth_token: String,
  pub method: OperationMethodType,
  pub trees: Option<Vec<HashTree>>, // vector of tree names
  pub key: Option<String>, // key name
  pub data: Option<String>, // data to be stored (generic, usually json endpoint of a tree)
}