
use std::{fs, path::Path};


#[allow(dead_code)]
#[derive(schemars::JsonSchema)]
struct GeneratedRootSchema {
  // http protocol
  role: protocol::schemas::Role,
  register_request: protocol::schemas::http_protocol::RegisterRequest,
  register_response: protocol::schemas::http_protocol::RegisterResponse,
  role_authenticate_request: protocol::schemas::http_protocol::RoleAuthenticateRequest,

  // socket protocol (message)
  echo_tree_client_socket_message: protocol::schemas::socket_protocol::client_socket_protocol::EchoTreeClientSocketMessage,
  echo_tree_server_socket_message: protocol::schemas::socket_protocol::server_socket_protocol::EchoTreeServerSocketMessage,

  // server message protocols
  echo_tree_event: protocol::schemas::socket_protocol::server_socket_protocol::EchoTreeEvent,
  echo_item_event: protocol::schemas::socket_protocol::server_socket_protocol::EchoItemEvent,
  response_event: protocol::schemas::socket_protocol::server_socket_protocol::StatusResponseEvent,

  // client message protocols
  checksum_event: protocol::schemas::socket_protocol::client_socket_protocol::ChecksumEvent,
  set_event: protocol::schemas::socket_protocol::client_socket_protocol::InsertEvent,
  get_event: protocol::schemas::socket_protocol::client_socket_protocol::GetEvent,
  delete_event: protocol::schemas::socket_protocol::client_socket_protocol::DeleteEvent,

  set_tree_event: protocol::schemas::socket_protocol::client_socket_protocol::SetTreeEvent,
  get_tree_event: protocol::schemas::socket_protocol::client_socket_protocol::GetTreeEvent,

  subscribe_event: protocol::schemas::socket_protocol::client_socket_protocol::SubscribeEvent,
  unsubscribe_event: protocol::schemas::socket_protocol::client_socket_protocol::UnsubscribeEvent,

  // just a test
  test_struct: protocol::schemas::TestStruct,
}

fn main() {
  // Generate schema.json in the root repository

  let schema_file = Path::new("../schema/schema.json");
  let root_schema = schemars::schema_for!(GeneratedRootSchema);

  let schema_json = serde_json::to_string_pretty(&root_schema).unwrap();
  fs::write(schema_file, schema_json).unwrap();
}