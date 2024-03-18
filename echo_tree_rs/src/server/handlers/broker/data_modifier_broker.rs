use protocol::schemas::socket_protocol::{EchoEvent, EchoMethodType, HashTree, OperationRequest};

use crate::common::{Clients, EchoDB};


pub async fn set_broker(uuid: String, msg: OperationRequest, clients: &Clients, db: &EchoDB) {
  let hash_trees = match msg.trees {
    Some(t) => t,
    None => {
      log::warn!("{}: no trees to set", uuid);
      return;
    }
  };

  let key = match msg.key {
    Some(k) => k,
    None => {
      log::warn!("{}: no key to set", uuid);
      return;
    }
  };

  let data = match msg.data {
    Some(d) => d,
    None => {
      log::warn!("{}: no data to set", uuid);
      return;
    }
  };

  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let mut write_db = db.write().await;

  for hash_tree in hash_trees.iter() {
    if client.get_accessible_subscribed_trees().contains(&hash_tree.tree) {
      let tree = match write_db.get_tree_map_mut().get_tree_mut(hash_tree.tree.to_string()) {
        Some(t) => t,
        None => {
          log::warn!("{}: tree not found: {}", uuid, hash_tree.tree);
          continue;
        }
      };

      match tree.insert(key.as_bytes(), data.as_bytes()) {
        Ok(v) => {
          log::debug!("{}: inserted data: {:?} into tree: {}", uuid, v, hash_tree.tree);
          // send echo
          let echo_event = EchoEvent {
            method: EchoMethodType::EchoItem,
            trees: Some(vec![
              HashTree {
                tree: hash_tree.tree.clone(),
                checksum: Some(tree.get_checksum())
              }
            ]),
            key: Some(key.clone()),
            data: Some(data.clone()),
          };

          client.echo_client(echo_event);
        },
        Err(e) => {
          log::warn!("{}: insert failed for {}: {}", uuid, hash_tree.tree, e);
        }
      }
    }
  }
}

pub async fn get_broker(uuid: String, msg: OperationRequest, clients: &Clients, db: &EchoDB) {
  let hash_trees = match msg.trees {
    Some(t) => t,
    None => {
      log::warn!("{}: no trees to get", uuid);
      return;
    }
  };

  let key = match msg.key {
    Some(k) => k,
    None => {
      log::warn!("{}: no key to get", uuid);
      return;
    }
  };

  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let read_db = db.read().await;

  for hash_tree in hash_trees.iter() {
    if client.get_accessible_subscribed_trees().contains(&hash_tree.tree) {
      let tree = match read_db.get_tree_map().get_tree(hash_tree.tree.to_string()) {
        Some(t) => t,
        None => {
          log::warn!("{}: tree not found: {}", uuid, hash_tree.tree);
          continue;
        }
      };

      let string_data = match tree.get_string_data(key.clone()) {
        Ok(d) => d,
        Err(e) => {
          log::warn!("{}: get_string_data failed for {}: {}", uuid, hash_tree.tree, e);
          continue;
        }
      };

      // send echo
      let echo_event = EchoEvent {
        method: EchoMethodType::EchoItem,
        trees: Some(vec![
          HashTree {
            tree: hash_tree.tree.clone(),
            checksum: Some(tree.get_checksum())
          }
        ]),
        key: Some(key.clone()),
        data: Some(string_data),
      };

      client.echo_client(echo_event);
    }
  }
}