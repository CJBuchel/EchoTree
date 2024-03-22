use log::warn;
use protocol::schemas::socket_protocol::client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage};

use crate::common::{Clients, EchoDB};

mod subscribe_broker;
mod unsubscribe_broker;
mod checksum_broker;
mod insert_broker;
mod insert_tree_broker;

/**
 * Broker for the echo message
 * Breakout for each echo message method
 */
pub async fn echo_message_broker(uuid:String, msg: EchoTreeClientSocketMessage, clients: &Clients, db: &EchoDB) {
  match msg.message_event {
    EchoTreeClientSocketEvent::SubscribeEvent => {
      subscribe_broker::subscribe_broker(uuid, msg, clients).await;
    },
    EchoTreeClientSocketEvent::UnsubscribeEvent => {
      unsubscribe_broker::unsubscribe_broker(uuid, msg, clients).await;
    },
    EchoTreeClientSocketEvent::ChecksumEvent => {
      checksum_broker::checksum_broker(uuid, msg, clients, db).await;
    },
    EchoTreeClientSocketEvent::InsertEvent => {
      insert_broker::insert_broker(uuid, msg, clients, db).await;
    },
    EchoTreeClientSocketEvent::SetTreeEvent => {
      insert_tree_broker::set_tree_broker(uuid, msg, clients, db).await;
    },
    _ => {
      warn!("{}: unhandled method", uuid);
    },
  }
}