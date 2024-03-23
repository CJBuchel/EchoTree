use futures::{FutureExt, StreamExt};
use log::{debug, error, info, warn};
use protocol::schemas::socket_protocol::client_socket_protocol::EchoTreeClientSocketMessage;

use crate::{common::{client::Client, ClientMap, EchoDB, ResponseResult}, server::handlers::broker::echo_message_broker};

async fn check_client_auth(uuid: String, auth_token: String, clients: &ClientMap) -> bool {
  let client = match clients.read().await.get(&uuid) {
    Some(c) => c.clone(),
    None => {
      error!("{}: client not found", uuid);
      return false;
    }
  };

  if client.auth_token != auth_token {
    warn!("{}: auth token mismatch", uuid);
    return false;
  }

  true
}

async fn client_msg(uuid: String, msg: warp::filters::ws::Message, clients: &ClientMap, db: &EchoDB) {
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

  let operation_request: EchoTreeClientSocketMessage = match serde_json::from_str(message) {
    Ok(v) => v,
    Err(e) => {
      error!("{}: {:?}", uuid, e);
      return;
    }
  };

  // check auth code
  if !check_client_auth(uuid.clone(), operation_request.auth_token.clone(), clients).await {
    return;
  }

  // match the method protocol
  echo_message_broker(uuid, operation_request, clients, db).await;
}

async fn client_connection(ws: warp::ws::WebSocket, uuid: String, clients: ClientMap, mut client: Client, db: EchoDB) {
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
    client_msg(uuid.clone(), msg, &clients, &db).await;
  }

  clients.write().await.remove(&uuid);
  info!("{} disconnected", uuid);
}

// -> ResponseResult<impl warp::Reply>
pub async fn ws_handler(ws: warp::ws::Ws, uuid: String, clients: ClientMap, db: EchoDB) -> ResponseResult<impl warp::Reply>  {
  let client = clients.read().await.get(&uuid).cloned();

  match client {

    // client found
    Some(c) => {
      Ok(ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c, db)))
    },

    // client not found
    None => {
      Err(warp::reject::not_found())
    }
  }
}