use std::collections::HashMap;

use schemars::JsonSchema;

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoTreeEventTree {
  pub tree_name: String, // tree name
  pub checksum: u32, // checksum of the tree (not the key/data)
  pub tree: HashMap<String, String>, // (k, v)
}

#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoTreeEvent {
  pub trees: Vec<EchoTreeEventTree>, // trees
}


#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoItemEvent {
  pub checksum: u32, // new checksum of the tree (not the key/data)
  pub tree_name: String, // tree name
  pub key: String, // key name
  pub data: String, // data
}


/**
 * Echo Tree Event
 * dictates the message structure, i.e:
 * - PingEvent: (no message)
 * - EchoTreeEvent: trees, data
 * - EchoItemEvent: tree, key, data
 *   etc...
 */
#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub enum EchoTreeServerSocketEvent {
  PingEvent, // (no message)
  EchoTreeEvent, // trees, data
  EchoItemEvent, // tree, key, data
}


/**
 * Echo Tree Server Socket Message
 * message to be sent to the client (json data, represented by the event type)
 */
#[derive(serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct EchoTreeServerSocketMessage {
  pub auth_token: String, // auth token for the client (optional for the client to verify the message is from the server)
  pub message_event: EchoTreeServerSocketEvent, // message type, dictates the message structure.
  pub message: Option<String>, // message to be sent to the client (json data, represented by the message type)
}