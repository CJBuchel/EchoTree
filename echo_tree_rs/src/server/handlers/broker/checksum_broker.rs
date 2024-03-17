use protocol::schemas::socket_protocol::EchoEvent;

use crate::common::{Clients, EchoDB};


pub async fn checksum_broker(uuid: String, msg: EchoEvent, clients: &Clients, db: &EchoDB) {
  // client trees

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
    if client.echo_trees.contains(stale_tree) {
      // client is subscribed to this stale tree, will send an update
      if client.role_trees.contains(stale_tree) { // if client has access to this tree
        // @TODO
      }
    }
  }
}