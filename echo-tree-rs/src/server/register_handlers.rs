use log::debug;

use crate::protocol::{Client, Clients, RegisterRequest, RegisterResponse, ResponseResult};

async fn register_client(uuid: String, auth_token:String, echo_trees: Vec<String>, clients: Clients) {

  debug!("registering client with uuid: {}", uuid);
  clients.write().await.insert(uuid, Client{
    auth_token,
    echo_trees,
    sender: None,
  });
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> ResponseResult<impl warp::reply::Reply> {
  let uuid = uuid::Uuid::new_v4().to_string();
  let auth_token = uuid::Uuid::new_v4().to_string();

  // check username and password here... (@TODO)

  register_client(uuid.clone(), auth_token.clone(), body.echo_trees, clients).await;

  Ok(
    warp::reply::json(&RegisterResponse {
      url: format!("wss://localhost:2121/{}", uuid),
      auth_token,
    })
  )
}

pub async fn unregister_handler(uuid: String, clients: Clients) -> ResponseResult<impl warp::reply::Reply> {
  debug!("un-registering client with uuid: {}", uuid);
  clients.write().await.remove(&uuid);
  Ok(warp::http::StatusCode::OK)
}

pub fn pulse_handler() -> impl futures::Future<Output = ResponseResult<impl warp::reply::Reply>> {
  debug!("pulse");
  futures::future::ready(Ok(warp::http::StatusCode::OK))
}