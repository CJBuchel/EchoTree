use log::{debug, error, warn};
use protocol::schemas::socket_protocol::server_socket_protocol::{
  EchoItemEvent, EchoTreeEvent, EchoTreeEventTree, EchoTreeServerSocketEvent, EchoTreeServerSocketMessage, StatusResponseEvent
};
use tokio::sync::mpsc;
use warp::{filters::ws::Message, Filter};

use crate::db::db::Database;

pub type ResponseResult<T> = std::result::Result<T, warp::reject::Rejection>;
pub type ClientHashMap = std::collections::HashMap<String, Client>;
pub type Clients = std::sync::Arc<tokio::sync::RwLock<ClientHashMap>>; // clients id, client
pub type EchoDB = std::sync::Arc<tokio::sync::RwLock<Database>>;

pub trait ClientsExtended {
  fn echo_tree_clients(&self, msg: Vec<EchoTreeEventTree>);
  fn echo_item_clients(&self, msg: EchoItemEvent);
}

impl ClientsExtended for ClientHashMap {
  fn echo_tree_clients(&self, msg: Vec<EchoTreeEventTree>) {
    for (_, client) in self.iter() {
      // prepare the trees for specific clients
      let client_filtered_role_trees: Vec<EchoTreeEventTree> = msg
        .iter()
        .filter(|t| client.is_accessible_subscribed_tree(&t.tree_name))
        .map(|t| t.clone())
        .collect();
      
      // prepare the echo message
      let echo_message = EchoTreeServerSocketMessage {
        auth_token: client.auth_token.clone(),
        message_event: EchoTreeServerSocketEvent::EchoTreeEvent,
        message: Some(serde_json::to_string(&EchoTreeEvent {
          trees: client_filtered_role_trees,
        }).unwrap_or_default()),
      };

      // send the message to the client/echo the event
      client.echo_client(echo_message);
    }
  }

  fn echo_item_clients(&self, msg: EchoItemEvent) {
    for (_, client) in self.iter() {
      if client.can_access_tree(&msg.tree_name) {
        let echo_message = EchoTreeServerSocketMessage {
          auth_token: client.auth_token.clone(),
          message_event: EchoTreeServerSocketEvent::EchoItemEvent,
          message: Some(serde_json::to_string(&msg).unwrap_or_default()),
        };

        client.echo_client(echo_message);
      }
    }
  }
}

pub fn with_clients(
  clients: Clients,
) -> impl warp::Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
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
  pub echo_trees: Vec<String>,      // list of topics/trees the client is subscribed to
  pub sender: Option<ClientSender>, // sender channel to this client
}

impl Client {
  pub fn new(
    auth_token: String,
    role_trees: Vec<String>,
    echo_trees: Vec<String>,
    sender: Option<ClientSender>,
  ) -> Self {
    Client {
      auth_token,
      role_trees,
      echo_trees,
      sender,
    }
  }
  // send a message to the client
  pub fn send_message(&self, msg: String) {
    // send message to client
    if let Some(sender) = &self.sender {
      match sender.send(Ok(Message::text(msg))) {
        Ok(_) => debug!("message sent to client"),
        Err(e) => error!("error sending message to client: {}", e),
      }
    }
  }

  // send an echo event to the client
  pub fn echo_client(&self, msg: EchoTreeServerSocketMessage) {
    let json = match serde_json::to_string(&msg) {
      Ok(j) => j,
      Err(e) => {
        error!("error serializing echo event: {}", e);
        return;
      }
    };

    self.send_message(json);
  }

  pub fn respond(&self, res: StatusResponseEvent) {
    let echo_message = EchoTreeServerSocketMessage {
      auth_token: self.auth_token.clone(),
      message_event: EchoTreeServerSocketEvent::StatusResponseEvent,
      message: Some(serde_json::to_string(&res).unwrap_or_default()),
    };

    self.echo_client(echo_message);
  }

  // check if this client has access to a tree
  pub fn can_access_tree(&self, tree: &str) -> bool {
    // access is based on file system standards. If a user has access to `/a/` then they have access to `/a/b/` but not /b/
    self.role_trees.iter().any(|t| tree.starts_with(t))
  }

  // check if this client is subscribed to a tree
  pub fn is_subscribed_to_tree(&self, tree: &str) -> bool {
    // access is based on file system standards. If a user has access to `/a/` then they have access to `/a/b/` but not /b/
    self.echo_trees.iter().any(|t| tree.starts_with(t))
  }

  // check if this client has access to a tree and is subscribed to it
  pub fn is_accessible_subscribed_tree(&self, tree: &str) -> bool {
    self.can_access_tree(tree) && self.is_subscribed_to_tree(tree)
  }

  // returns the accessible trees of the given trees
  pub fn get_accessible_trees(&self, trees: Vec<String>) -> Vec<String> {
    trees
      .iter()
      .filter(|t| self.can_access_tree(t))
      .map(|t| t.to_string())
      .collect()
  }

  // returns the unauthorized trees of the given trees
  pub fn get_unauthorized_trees(&self, trees: Vec<String>) -> Vec<String> {
    trees
      .iter()
      .filter(|t| !self.can_access_tree(t))
      .map(|t| t.to_string())
      .collect()
  }

    // returns the clients accessible & subscribed trees
  pub fn get_accessible_subscribed_trees(&self) -> Vec<String> {
    // get the intersection of role_trees and echo_trees
    self
      .role_trees
      .iter()
      .filter(|t| self.can_access_tree(&t))
      .map(|t| t.to_string())
      .collect()
  }
}
