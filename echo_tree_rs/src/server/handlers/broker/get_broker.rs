use protocol::schemas::socket_protocol::{client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage, GetEvent}, server_socket_protocol::{EchoItemEvent, EchoTreeServerSocketEvent, EchoTreeServerSocketMessage, StatusResponseEvent}};

use crate::common::{Clients, EchoDB};


pub async fn get_broker(uuid: String, msg: EchoTreeClientSocketMessage, clients: &Clients, db: &EchoDB) {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      log::warn!("{}: client not found", uuid);
      return;
    }
  };

  let msg: GetEvent = match serde_json::from_str(&msg.message.unwrap_or("".to_string())) {
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

  // check if client has access to the tree
  if client.can_access_tree(&msg.tree_name) {
    // db access
    let read_db = db.read().await;
    let res = read_db.get(msg.tree_name.clone(), msg.key.clone());
    let checksum = match read_db.get_tree_map().get_tree(msg.tree_name.clone()) {
      Some(t) => t.get_checksum(),
      None => 0,
    };

    let res = match res {
      Some(r) => r,
      None => {
        client.respond(StatusResponseEvent {
          status_code: warp::http::StatusCode::NOT_FOUND.as_u16(),
          from_event: Some(EchoTreeClientSocketEvent::GetEvent),
          message: Some("not found".to_string()),
        });
        return;
      }
    };
    
    let echo_event = EchoItemEvent {
      checksum,
      tree_name: msg.tree_name,
      key: msg.key,
      data: res,
    };

    client.echo_client(EchoTreeServerSocketMessage {
      auth_token: client.auth_token.clone(),
      message_event: EchoTreeServerSocketEvent::EchoItemEvent,
      message: Some(serde_json::to_string(&echo_event).unwrap_or_default()),
    });

    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::OK.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::GetEvent),
      message: Some("ok".to_string()),
    });

  } else {
    log::debug!("{}: client does not have access to tree: {}", uuid, msg.tree_name);
    client.respond(StatusResponseEvent {
      status_code: warp::http::StatusCode::UNAUTHORIZED.as_u16(),
      from_event: Some(EchoTreeClientSocketEvent::GetEvent),
      message: Some(format!("client does not have access to tree: {}", msg.tree_name)),
    });
    return;
  }
}