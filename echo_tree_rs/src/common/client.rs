use log::{debug, error};
use protocol::schemas::socket_protocol::server_socket_protocol::{EchoTreeServerSocketEvent, EchoTreeServerSocketMessage, StatusResponseEvent};
use tokio::sync::mpsc;
use warp::filters::ws::Message;

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
      match sender.send(Ok(Message::text(msg.clone()))) {
        Ok(_) => debug!("message sent to client: {}", msg),
        Err(e) => error!("error sending message to client: {}", e),
      }
    }
  }

  pub fn respond(&self, res: StatusResponseEvent) {
    let echo_message = EchoTreeServerSocketMessage {
      auth_token: self.auth_token.clone(),
      message_event: EchoTreeServerSocketEvent::StatusResponseEvent,
      message: Some(serde_json::to_string(&res).unwrap_or_default()),
    };

    // send the message to the client/echo the event
    let json = serde_json::to_string(&echo_message).unwrap_or_default();
    self.send_message(json);
  }

  // check if this client has access to a tree
  pub fn has_access_to_tree(&self, tree: &str) -> bool {
    // access is based on file system standards. If a user has access to `/a/` then they have access to `/a/b/` but not /b/
    self.role_trees.iter().any(|t| tree.starts_with(t))
  }

  // check if this client is subscribed to a tree
  pub fn is_subscribed_to_tree(&self, tree: &str) -> bool {
    // access is based on file system standards. If a user has access to `/a/` then they have access to `/a/b/` but not /b/
    self.echo_trees.iter().any(|t| tree.starts_with(t))
  }

  // check if this client has access to a tree and is subscribed to it
  pub fn has_access_and_subscribed_to_tree(&self, tree: &str) -> bool {
    self.has_access_to_tree(tree) && self.is_subscribed_to_tree(tree)
  }

  // filters out the trees that the client has access to
  pub fn filter_accessible_trees(&self, trees: Vec<String>) -> Vec<String> {
    trees
      .iter()
      .filter(|t| self.has_access_to_tree(t))
      .map(|t| t.to_string())
      .collect()
  }

  // returns the unauthorized trees of the given trees
  pub fn filter_unauthorized_trees(&self, trees: Vec<String>) -> Vec<String> {
    trees
      .iter()
      .filter(|t| !self.has_access_to_tree(t))
      .map(|t| t.to_string())
      .collect()
  }

  // filters out the trees that the client is subscribed to and has access to
  pub fn filter_accessible_and_subscribed_trees(&self, trees: Vec<String>) -> Vec<String> {
    trees
      .iter()
      .filter(|t| self.has_access_and_subscribed_to_tree(t))
      .map(|t| t.to_string())
      .collect()
  }

  // returns the clients accessible & subscribed trees
  pub fn get_accessible_and_subscribed_trees(&self) -> Vec<String> {
    // get the intersection of role_trees and echo_trees
    self
      .echo_trees
      .iter()
      .filter(|t| self.has_access_to_tree(&t))
      .map(|t| t.to_string())
      .collect()
  }
}