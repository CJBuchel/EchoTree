use log::{debug, info};
use protocol::schemas::socket_protocol::{client_socket_protocol::{ChecksumEvent, EchoTreeClientSocketEvent, EchoTreeClientSocketMessage}, server_socket_protocol::{EchoTreeEventTree, StatusResponseEvent}};

use crate::common::{client_echo::ClientEcho, ClientMap, EchoDB};


pub async fn checksum_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &ClientMap, db: &EchoDB) {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };
  
  let msg: ChecksumEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      log::error!("{}: {:?}", uuid, e);
      client.respond(StatusResponseEvent {
        status_code: warp::http::StatusCode::BAD_REQUEST.as_u16(),
        from_event: Some(EchoTreeClientSocketEvent::ChecksumEvent),
        message: Some(format!("{:?}", e)),
      });
      return;
    }
  };

  // check the checksums against the db trees
  let read_db = db.read().await;

  let new_client_trees: Vec<EchoTreeEventTree> = msg.tree_checksums.iter().filter_map(|(tree_name, checksum)| { // filter_map is a combination of filter and map
    let tree = read_db.get_tree_map().get_tree(tree_name.to_string())?;
    if tree.get_checksum() != *checksum { // if the tree checksum does not match the checksum from the client
      log::error!("{}: tree checksum mismatch: {} != {}", uuid, tree.get_checksum(), checksum);
      
      if client.has_access_and_subscribed_to_tree(tree_name) { // if the client has access to the tree
        let tree_hashmap = tree.get_as_hashmap().ok()?;
        Some(EchoTreeEventTree { // return the tree name, the new tree as a hashmap, and the new tree checksum
          tree_name: tree_name.clone(),
          tree: tree_hashmap,
        })
      } else {
        None
      }
    } else {
      info!("{}: good checksum: server sum: {} client: {}", uuid, tree.get_checksum(), checksum);
      None
    }
  }).collect();

  // echo the new trees to the client
  if !new_client_trees.is_empty() {
    client.echo_tree(new_client_trees);
  }

  client.respond(StatusResponseEvent {
    status_code: warp::http::StatusCode::OK.as_u16(),
    from_event: Some(EchoTreeClientSocketEvent::ChecksumEvent),
    message: Some("checksums checked".to_string()),
  });
}