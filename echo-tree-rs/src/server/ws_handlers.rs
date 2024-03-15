use futures::{FutureExt, StreamExt};

use crate::protocol::{Client, Clients, ResponseResult};

async fn client_msg(uuid: String, msg: warp::filters::ws::Message, clients: &Clients) {
  log::debug!("{}: {:?}", uuid, msg);

  let message = match msg.to_str() {
    Ok(v) => v,
    Err(e) => {
      log::error!("{}: {:?}", uuid, e);
      return;
    }
  };

  if message == "ping" || message == "ping\n" {
    return;
  }

  // @TODO: handle client messages
}

async fn client_connection(ws: warp::ws::WebSocket, uuid: String, clients: Clients, mut client: Client) {
  let (client_ws_sender, mut client_ws_recv) = ws.split();
  let (client_sender, client_recv) = tokio::sync::mpsc::unbounded_channel();

  let client_recv = tokio_stream::wrappers::UnboundedReceiverStream::new(client_recv);
  tokio::task::spawn(client_recv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      log::error!("websocket send error: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(uuid.clone(), client);

  log::info!("{} connected", uuid);

  while let Some(result) = client_ws_recv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        log::error!("{}: {:?}", uuid, e);
        break;
      }
    };

    // client message
    client_msg(uuid.clone(), msg, &clients).await;
  }

  clients.write().await.remove(&uuid);
  log::info!("{} disconnected", uuid);
}

// -> ResponseResult<impl warp::Reply>
pub async fn ws_handler(ws: warp::ws::Ws, uuid: String, clients: Clients) -> ResponseResult<impl warp::Reply>  {
  let client = clients.read().await.get(&uuid).cloned();

  match client {

    // client found
    Some(c) => {
      Ok(
        ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c))
      )
    },

    // client not found
    None => {
      Err(warp::reject::not_found())
    }
  }
}