

use tokio::sync::mpsc;
use warp::{filters::ws::Message, Filter};


/**
 * Client structure,
 * stores information about a client as a temporary connection
 */
#[derive(Clone)]
pub struct Client {
  pub auth_token: String, // unique authentication for the client. Used to check if the client is authorized to access certain echo trees
  pub echo_trees: Vec<String>, // list of topics/trees the client is subscribed to
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>, // sender channel to this client
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
  pub echo_trees: Vec<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
  pub url: String,
  pub auth_token: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum MethodProtocol {
  Echo,
  Set,
  Get,
  Delete,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EchoEvent {
  pub auth_token: String,
  pub method: MethodProtocol,
  pub tree: String,
  pub data: String,
}

pub type ResponseResult<T> = std::result::Result<T, warp::reject::Rejection>;
pub type Clients = std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Client>>>;

pub fn with_clients(clients: Clients) -> impl warp::Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || clients.clone())
}