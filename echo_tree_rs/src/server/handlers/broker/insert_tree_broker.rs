use log::warn;
use protocol::schemas::socket_protocol::{
  client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage, SetTreeEvent},
  server_socket_protocol::StatusResponseEvent,
};

use crate::common::{Clients, EchoDB};

pub async fn set_tree_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &Clients, db: &EchoDB) {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let msg: SetTreeEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      log::warn!("{}: {:?}", uuid, e);
      client.respond(StatusResponseEvent {
        status_code: warp::http::StatusCode::BAD_REQUEST.as_u16(),
        from_event: Some(EchoTreeClientSocketEvent::SetTreeEvent),
        message: Some(format!("{:?}", e)),
      });
      return;
    }
  };

  // create list of tree names the client is trying to access
  let tree_names: Vec<String> = msg.trees.iter().map(|(t, _)| t.clone()).collect();
  let unauthorized_tree_names: Vec<String> = client.get_unauthorized_trees(tree_names.clone());

  // access db and set trees the client has access to
  let mut write_db = db.write().await;
  for (tree_name, tree) in msg.trees {
    if client.can_access_tree(&tree_name) {
      let managed_tree = match write_db.get_tree_map_mut().get_tree_mut(tree_name.clone()) {
        Some(t) => t,
        None => {
          warn!("{}: tree not found: {}", uuid, tree_name);
          continue;
        }
      };

      match managed_tree.set_from_hashmap(tree) {
        Ok(_) => log::debug!("{}: tree set: {}", uuid, tree_name),
        Err(e) => log::error!("{}: error setting tree: {}", uuid, e),
      }
    }
  }

  if unauthorized_tree_names.is_empty() {
    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::OK.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::SetTreeEvent),
      message: Some("trees set".to_string()),
    });
  } else {
    log::debug!(
      "{}: client does not have access to trees: {:?}",
      uuid,
      unauthorized_tree_names
    );
    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::UNAUTHORIZED.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::SetTreeEvent),
      message: Some(format!(
        "client does not have access to trees: {:?}",
        unauthorized_tree_names
      )),
    });
  }
}
