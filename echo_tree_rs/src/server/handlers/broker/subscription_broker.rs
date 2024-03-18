use log::debug;
use protocol::schemas::socket_protocol::OperationRequest;
use crate::common::Clients;


pub async fn subscribe_broker(uuid: String, msg: OperationRequest, clients: &Clients) {
  match msg.trees {
    Some(trees) => {
      let mut client = match clients.read().await.get(&uuid).cloned() {
        Some(c) => c,
        None => {
          debug!("{}: client not found", uuid);
          return;
        }
      };

      // add only the trees the client doesn't have
      for tree in trees {
        if !client.echo_trees.contains(&tree.tree) {
          client.echo_trees.push(tree.tree.clone());
        }
      }

      clients.write().await.insert(uuid.clone(), client);
    },
    None => {
      debug!("{}: no trees to subscribe to", uuid);
    }
  }
}

pub async fn unsubscribe_broker(uuid:String, msg: OperationRequest, clients: &Clients) {
  match msg.trees {
    Some(trees) => {
      let mut client = match clients.read().await.get(&uuid).cloned() {
        Some(c) => c,
        None => {
          debug!("{}: client not found", uuid);
          return;
        }
      };

      // remove only the trees the client has
      for tree in trees {
        if client.echo_trees.contains(&tree.tree) {
          client.echo_trees.retain(|t| t != &tree.tree);
        }
      }

      clients.write().await.insert(uuid.clone(), client);
    },
    None => {
      debug!("{}: no trees to unsubscribe from", uuid);
    }
  }
}