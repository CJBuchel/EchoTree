#[derive(serde::Deserialize, serde::Serialize)]
pub enum MethodProtocol {
  // generic methods
  Set(String, String, String), // tree, key, data
  Get(String, String), // tree, key
  Delete(String, String), // tree, key

  // echo sync methods
  Echo(String, String), // tree, data
  Subscribe(Vec<String>), // tree
  Unsubscribe(Vec<String>), // tree
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EchoEvent {
  pub auth_token: String,
  pub method: MethodProtocol,
}