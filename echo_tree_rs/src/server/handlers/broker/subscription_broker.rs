use log::debug;
use protocol::schemas::socket_protocol::EchoEvent;

use crate::common::Clients;


pub async fn subscribe_broker(uuid: String, msg: EchoEvent, clients: &Clients) {
  match msg.params.trees {
    Some(trees) => {
      debug!("{}: subscribing to trees: {:?}", uuid, trees);
      let mut client = match clients.read().await.get(&uuid).cloned() {
        Some(c) => c,
        None => {
          debug!("{}: client not found", uuid);
          return;
        }
      };

      // add only the trees the client doesn't have
      for tree in trees {
        if !client.echo_trees.contains(&tree) {
          client.echo_trees.push(tree.clone());
        }
      }

      clients.write().await.insert(uuid.clone(), client);
    },
    None => {
      debug!("{}: no trees to subscribe to", uuid);
    }
  }
}

pub async fn unsubscribe_broker(uuid:String, msg: EchoEvent, clients: &Clients) {
  match msg.params.trees {
    Some(trees) => {
      debug!("{}: unsubscribing from trees: {:?}", uuid, trees);
      let mut client = match clients.read().await.get(&uuid).cloned() {
        Some(c) => c,
        None => {
          debug!("{}: client not found", uuid);
          return;
        }
      };

      // remove only the trees the client has
      for tree in trees {
        if client.echo_trees.contains(&tree) {
          client.echo_trees.retain(|t| t != &tree);
        }
      }

      clients.write().await.insert(uuid.clone(), client);
    },
    None => {
      debug!("{}: no trees to unsubscribe from", uuid);
    }
  }
}