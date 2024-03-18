use protocol::schemas::socket_protocol::{Checksum, EchoEvent};

use crate::common::{Clients, EchoDB};


pub async fn checksum_broker(uuid: String, msg: EchoEvent, clients: &Clients, db: &EchoDB) {
  // client tree and checksums
  let checksums = match msg.params.checksums {
    Some(c) => c,
    None => {
      log::warn!("{}: no checksums to compare", uuid);
      return;
    }
  };

  // check the checksums against the db trees
  let read_db = db.read().await;
  let mut stale_trees: Vec<String> = Vec::new();

  for checksum in checksums.iter() {
    let tree = match read_db.get_tree_map().get_tree(checksum.tree.clone()) {
      Some(t) => t,
      None => {
        log::debug!("{}: tree not found: {}", uuid, checksum.tree);
        continue;
      }
    };

    // check the tree checksum against client checksum
    if tree.get_checksum() != checksum.checksum {
      log::debug!("{}: tree checksum mismatch: {} != {}", uuid, tree.get_checksum(), checksum.checksum);
      stale_trees.push(checksum.tree.clone());
    }
  }

  // find the matching trees for the client
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  for stale_tree in stale_trees.iter() {
    // client has access to this stale tree, will send an update
    if client.get_accessible_subscribed_trees().contains(stale_tree) {
      // serialize tree into json and send to client
      let tree = match read_db.get_tree_map().get_tree(stale_tree.to_string()) {
        Some(t) => t,
        None => {
          log::warn!("{}: tree not found: {}", uuid, stale_tree);
          continue;
        }
      };
      
      // convert tree to json using serde
      let tree_json = match tree.get_json() {
        Ok(j) => j,
        Err(e) => {
          log::error!("{}: get_json failed for {}: {}", uuid, stale_tree, e);
          continue;
        }
      };
  
      // prepare echo event
      let echo_event = EchoEvent {
        auth_token: client.auth_token.clone(),
        method: protocol::schemas::socket_protocol::MethodType::EchoTree,
        params: protocol::schemas::socket_protocol::MethodParameters {
          trees: None,
          key: None,
          checksums: Some(vec![Checksum {
            tree: stale_tree.clone(),
            checksum: tree.get_checksum(),
          }]),
          data: Some(tree_json),
        },
      };
      
      // send echo event to client
      client.echo_client(echo_event);
    }
  }
}