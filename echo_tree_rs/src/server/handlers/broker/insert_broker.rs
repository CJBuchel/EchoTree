use protocol::schemas::socket_protocol::{client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage, InsertEvent}, server_socket_protocol::{EchoItemEvent, StatusResponseEvent}};

use crate::common::{ClientMap, EchoDB, client_echo::ClientEcho};


pub async fn insert_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &ClientMap, db: &EchoDB) {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let msg: InsertEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
    Ok(v) => v,
    Err(e) => {
      log::warn!("{}: {:?}", uuid, e);
      client.respond(StatusResponseEvent {
        status_code: warp::http::StatusCode::BAD_REQUEST.as_u16(),
        from_event: Some(EchoTreeClientSocketEvent::InsertEvent),
        message: Some(format!("{:?}", e)),
      });
      return;
    }
  };

  // check if client has access to the tree
  if client.has_access_to_tree(&msg.tree_name) {
    // db access
    let mut write_db = db.write().await;
    let res = write_db.insert(msg.tree_name.clone(), msg.key.clone(), msg.data.clone());

    let echo_event = EchoItemEvent {
      tree_name: msg.tree_name.clone(),
      key: msg.key.clone(),
      data: msg.data.clone(),
    };

    // echo the event to sll subscribed clients
    clients.read().await.echo_item(echo_event);

    // respond to the client success
    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::OK.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::InsertEvent),
      message: res,
    });
  } else {
    log::debug!("{}: client does not have access to tree: {}", uuid, msg.tree_name);
    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::UNAUTHORIZED.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::InsertEvent),
      message: Some(format!("client does not have access to tree: {}", msg.tree_name)),
    });
    return;
  }
}