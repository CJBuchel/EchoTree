use log::{debug, error};
use protocol::schemas::http_protocol::{RegisterRequest, RegisterResponse};

use crate::common::{client::Client, ClientMap, EchoDB, ResponseResult};

async fn register_client(uuid: String, auth_token:String, subscribed_trees: Vec<String>, role_read_trees: Vec<String>, role_read_write_trees: Vec<String>, clients: ClientMap) {

  debug!("registering client with uuid: {}", uuid);
  clients.write().await.insert(uuid, Client::new(auth_token, role_read_trees, role_read_write_trees, subscribed_trees, None));
}

pub async fn register_handler(body: RegisterRequest, clients: ClientMap, database: EchoDB, port: u16, ws_protocol: String) -> ResponseResult<impl warp::reply::Reply> {
  let uuid = uuid::Uuid::new_v4().to_string();
  let auth_token = uuid::Uuid::new_v4().to_string();

  let db = database.read().await;

  // check authentication
  let role_id = body.role_id.clone().unwrap_or("".to_string());
  let password = body.password.clone().unwrap_or("".to_string());

  let role_read_trees = match db.get_role_manager().authenticate_role(role_id.clone(), password.clone()) {
    true => db.get_role_manager().get_role_read_access(role_id.clone()),
    false => vec![],
  };

  let role_read_write_trees = match db.get_role_manager().authenticate_role(role_id.clone(), password) {
    true => db.get_role_manager().get_role_read_write_access(role_id),
    false => vec![],
  };

  register_client(uuid.clone(), auth_token.clone(), body.echo_trees, role_read_trees, role_read_write_trees, clients).await;

  let local_ip = local_ip_address::local_ip().unwrap();
  let url = format!("{}://{}:{}/echo_tree/ws/{}", ws_protocol, local_ip, port, uuid);

  let hierarchy = match db.get_hierarchy().get_as_hashmap() {
    Ok(h) => h,
    Err(e) => {
      error!("get_as_hashmap failed: {}", e);
      std::collections::HashMap::new()
    }
  };

  Ok(
    warp::reply::json(&RegisterResponse {
      uuid, // used to connect to the websocket, i.e ws://localhost:2121/ws/{uuid}
      url,
      auth_token,
      hierarchy,
    })
  )
}

pub async fn unregister_handler(client_uuid: String, clients: ClientMap) -> ResponseResult<impl warp::reply::Reply> {
  debug!("un-registering client with uuid: {}", client_uuid);
  clients.write().await.remove(&client_uuid);
  Ok(warp::http::StatusCode::OK)
}