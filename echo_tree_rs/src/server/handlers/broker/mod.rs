use log::warn;
use protocol::schemas::socket_protocol::client_socket_protocol::{EchoTreeClientSocketEvent, EchoTreeClientSocketMessage};

use crate::common::{Clients, EchoDB};

mod subscription_broker;
mod checksum_broker;
mod data_modifier_broker;

/**
 * Broker for the echo message
 * Breakout for each echo message method
 */
pub async fn echo_message_broker(uuid:String, msg: EchoTreeClientSocketMessage, clients: &Clients, db: &EchoDB) {
  match msg.message_event {
    EchoTreeClientSocketEvent::SubscribeEvent => {
      subscription_broker::subscribe_broker(uuid, msg, clients).await;
    },
    EchoTreeClientSocketEvent::UnsubscribeEvent => {
      subscription_broker::unsubscribe_broker(uuid, msg, clients).await;
    },
    EchoTreeClientSocketEvent::ChecksumEvent => {
      checksum_broker::checksum_broker(uuid, msg, clients, db).await;
    },
    // EchoTreeClientSocketEvent::SetEvent => {
    //   data_modifier_broker::set_broker(uuid, msg, clients, db).await;
    // },
    // EchoTreeClientSocketEvent::GetEvent => {
    //   data_modifier_broker::get_broker(uuid, msg, clients, db).await;
    // },
    _ => {
      warn!("{}: unhandled method", uuid);
    },
  }
}