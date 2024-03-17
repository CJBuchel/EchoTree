
use std::{fs, path::Path};

use protocol::schemas::{http_protocol::{RegisterRequest, RegisterResponse, RoleAuthenticateRequest}, socket_protocol::EchoEvent, Role};

#[allow(dead_code)]
#[derive(schemars::JsonSchema)]
struct GeneratedRootSchema {
  role: Role,
  register_request: RegisterRequest,
  register_response: RegisterResponse,
  role_authenticate_request: RoleAuthenticateRequest,
  echo_event: EchoEvent,
}

fn main() {
  // Generate schema.json in the root repository

  let schema_file = Path::new("../schema/schema.json");
  let root_schema = schemars::schema_for!(GeneratedRootSchema);

  let schema_json = serde_json::to_string_pretty(&root_schema).unwrap();
  fs::write(schema_file, schema_json).unwrap();
}