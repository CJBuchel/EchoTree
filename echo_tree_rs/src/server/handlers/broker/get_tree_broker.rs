use protocol::schemas::socket_protocol::{client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage, GetTreeEvent}, server_socket_protocol::{EchoTreeEventTree, StatusResponseEvent}};

use crate::common::{client_echo::ClientEcho, ClientMap, EchoDB};


pub async fn get_tree_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &ClientMap, db: &EchoDB) {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let msg: GetTreeEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      log::warn!("{}: {:?}", uuid, e);
      client.respond(StatusResponseEvent {
        status_code: warp::http::StatusCode::BAD_REQUEST.as_u16(),
        from_event: Some(EchoTreeClientSocketEvent::GetEvent),
        message: Some(format!("{:?}", e)),
      });
      return;
    }
  };

  // filter for accessible trees
  let accessible_trees = client.filter_accessible_trees(msg.tree_names);
  
  // get the trees
  let read_db = db.read().await;
  let trees: Vec<EchoTreeEventTree> = accessible_trees
    .iter()
    .filter_map(|t| {
      let tree = match read_db.get_tree_map().get_tree(t.clone()) {
        Some(v) => v,
        None => {
          log::warn!("{}: tree not found", uuid);
          return None;
        }
      };

      let tree_map = match tree.get_as_hashmap() {
        Ok(v) => v,
        Err(e) => {
          log::warn!("{}: {:?}", uuid, e);
          client.respond(StatusResponseEvent {
            status_code: warp::http::StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            from_event: Some(EchoTreeClientSocketEvent::GetEvent),
            message: Some(format!("{:?}", e)),
          });
          return None;
        }
      };

      Some(EchoTreeEventTree {
        tree_name: t.clone(),
        checksum: tree.get_checksum(),
        tree: tree_map,
      })
    })
    .collect();

  client.echo_tree(trees);

  client.respond(StatusResponseEvent {
    status_code: warp::http::StatusCode::OK.as_u16(),
    from_event: Some(EchoTreeClientSocketEvent::GetEvent),
    message: None,
  });
}