

use protocol::schemas::socket_protocol::{EchoEvent, OperationRequest};

use crate::common::{Clients, EchoDB};


pub async fn checksum_broker(uuid: String, msg: OperationRequest, clients: &Clients, db: &EchoDB) {
  // client tree and checksums
  let hash_trees = match msg.trees {
    Some(c) => c,
    None => {
      log::warn!("{}: no checksums to compare", uuid);
      return;
    }
  };

  // check the checksums against the db trees
  let read_db = db.read().await;
  let mut stale_trees: Vec<String> = Vec::new();

  for hash_tree in hash_trees.iter() {
    let tree = match read_db.get_tree_map().get_tree(hash_tree.tree.clone()) {
      Some(t) => t,
      None => {
        log::debug!("{}: tree not found: {}", uuid, hash_tree.tree);
        continue;
      }
    };

    let checksum = match hash_tree.checksum {
      Some(c) => c,
      None => {
        log::debug!("{}: checksum not found: {}", uuid, hash_tree.tree);
        continue;
      }
    };

    // check the tree checksum against client checksum
    if tree.get_checksum() != checksum {
      log::debug!("{}: tree checksum mismatch: {} != {}", uuid, tree.get_checksum(), checksum);
      stale_trees.push(hash_tree.tree.clone());
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
        method: protocol::schemas::socket_protocol::EchoMethodType::EchoTree,
        trees: Some(vec![
          protocol::schemas::socket_protocol::HashTree {
            tree: stale_tree.clone(),
            checksum: Some(tree.get_checksum()),
          }
        ]),
        key: None,
        data: Some(tree_json),
      };
      
      // send echo event to client
      client.echo_client(echo_event);
    }
  }
}