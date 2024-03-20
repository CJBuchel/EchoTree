use log::debug;
use protocol::schemas::socket_protocol::client_socket_protocol::{EchoTreeClientSocketMessage, SubscribeEvent, UnsubscribeEvent};
use crate::common::Clients;


pub async fn subscribe_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &Clients) {
  let msg: SubscribeEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      debug!("{}: {:?}", uuid, e);
      return;
    }
  };

  let mut client = match clients.read().await.get(&uuid).cloned() {
    Some(c) => c,
    None => {
      debug!("{}: client not found", uuid);
      return;
    }
  };

  let new_tree_names: Vec<String> = msg.tree_names.iter().filter(|tree| !client.echo_trees.contains(tree)).cloned().collect();
  client.echo_trees.extend(new_tree_names);

  clients.write().await.insert(uuid.clone(), client);
}

pub async fn unsubscribe_broker(uuid:String, msg: EchoTreeClientSocketMessage, clients: &Clients) {
  let msg: UnsubscribeEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      debug!("{}: {:?}", uuid, e);
      return;
    }
  };

  let mut client = match clients.read().await.get(&uuid).cloned() {
    Some(c) => c,
    None => {
      debug!("{}: client not found", uuid);
      return;
    }
  };

  // remove only the trees the client has
  let new_tree_names: Vec<String> = msg.tree_names.iter().filter(|tree| client.echo_trees.contains(tree)).cloned().collect();
  client.echo_trees.retain(|t| !new_tree_names.contains(t));

  clients.write().await.insert(uuid.clone(), client);
}