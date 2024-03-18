use log::{debug, error};
use protocol::schemas::socket_protocol::EchoEvent;
use tokio::sync::mpsc;
use warp::{filters::ws::Message, Filter};

use crate::db::db::Database;

pub type ResponseResult<T> = std::result::Result<T, warp::reject::Rejection>;
pub type Clients = std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, Client>>>; // clients id, client
pub type EchoDB = std::sync::Arc<tokio::sync::RwLock<Database>>;

pub fn with_clients(clients: Clients) -> impl warp::Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || clients.clone())
}

pub fn with_db(db: EchoDB) -> impl warp::Filter<Extract = (EchoDB,), Error = std::convert::Infallible> + Clone {
  warp::any().map(move || db.clone())
}

type ClientSender = mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;

/**
 * Client structure,
 * stores information about a client as a temporary connection
 */
#[derive(Clone)]
pub struct Client {
  // security/identification
  pub auth_token: String, // unique authentication for the client. Used to check if the client is authorized to access certain echo trees
  pub role_trees: Vec<String>, // list of topics/trees the client can access

  // client information
  pub echo_trees: Vec<String>, // list of topics/trees the client is subscribed to
  pub sender: Option<ClientSender>, // sender channel to this client
}

impl Client {
  pub fn new(auth_token: String, role_trees: Vec<String>, echo_trees: Vec<String>, sender: Option<ClientSender>) -> Self {
    Client {
      auth_token,
      role_trees,
      echo_trees,
      sender,
    }
  }

  pub fn get_accessible_subscribed_trees(&self) -> Vec<String> {
    // get the intersection of role_trees and echo_trees
    self.role_trees
      .iter()
      .filter(|t| self.echo_trees.contains(t))
      .map(|t| t.to_string())
      .collect()
  }

  pub fn send_message(&self, msg: String) {
    // @TODO
    // send message to client
    if let Some(sender) = &self.sender {
      match sender.send(Ok(Message::text(msg))) {
        Ok(_) => debug!("message sent to client"),
        Err(e) => error!("error sending message to client: {}", e),
      }
    }
  }

  pub fn echo_client(&self, msg: EchoEvent) {
    let json = match serde_json::to_string(&msg) {
      Ok(j) => j,
      Err(e) => {
        error!("error serializing echo event: {}", e);
        return;
      }
    };

    self.send_message(json);
  }
}