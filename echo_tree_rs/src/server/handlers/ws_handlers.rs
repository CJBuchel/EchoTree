use futures::{FutureExt, StreamExt};
use log::{debug, error, info};
use protocol::schemas::socket_protocol::{EchoEvent, MethodType};

use crate::common::{Client, Clients, ResponseResult};

async fn check_client_auth(uuid: String, auth_token: String, clients: &Clients) -> bool {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      error!("{}: client not found", uuid);
      return false;
    }
  };

  if client.auth_token != auth_token {
    error!("{}: auth token mismatch", uuid);
    return false;
  }

  true
}

async fn client_msg(uuid: String, msg: warp::filters::ws::Message, clients: &Clients) {
  debug!("{}: {:?}", uuid, msg);

  let message = match msg.to_str() {
    Ok(v) => v,
    Err(e) => {
      error!("{}: {:?}", uuid, e);
      return;
    }
  };

  if message == "ping" || message == "ping\n" {
    debug!("{}: pong", uuid);
    return;
  }

  let echo_event: EchoEvent = match serde_json::from_str(message) {
    Ok(v) => v,
    Err(e) => {
      error!("{}: {:?}", uuid, e);
      return;
    }
  };

  // check auth code
  if !check_client_auth(uuid.clone(), echo_event.auth_token, clients).await {
    return;
  }

  // match the method protocol
  match echo_event.method {
    MethodType::Subscribe => {
      // print the vector of trees
      debug!("{}: subscribe {:?}", uuid, echo_event.params.trees);
    },
    _ => {},
  }
}

async fn client_connection(ws: warp::ws::WebSocket, uuid: String, clients: Clients, mut client: Client) {
  let (client_ws_sender, mut client_ws_recv) = ws.split();
  let (client_sender, client_recv) = tokio::sync::mpsc::unbounded_channel();

  let client_recv = tokio_stream::wrappers::UnboundedReceiverStream::new(client_recv);
  tokio::task::spawn(client_recv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      error!("websocket send error: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(uuid.clone(), client);

  info!("{} connected", uuid);

  while let Some(result) = client_ws_recv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        error!("{}: {:?}", uuid, e);
        break;
      }
    };

    // client message
    client_msg(uuid.clone(), msg, &clients).await;
  }

  clients.write().await.remove(&uuid);
  info!("{} disconnected", uuid);
}

// -> ResponseResult<impl warp::Reply>
pub async fn ws_handler(ws: warp::ws::Ws, uuid: String, clients: Clients) -> ResponseResult<impl warp::Reply>  {
  let client = clients.read().await.get(&uuid).cloned();

  match client {

    // client found
    Some(c) => {
      Ok(ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c)))
    },

    // client not found
    None => {
      Err(warp::reject::not_found())
    }
  }
}