use log::warn;
use protocol::schemas::socket_protocol::{EchoEvent, MethodType};

use crate::common::{Clients, EchoDB};

mod subscription_broker;
mod checksum_broker;

/**
 * Broker for the echo message
 * Breakout for each echo message method
 */
pub async fn echo_message_broker(uuid:String, msg: EchoEvent, clients: &Clients, db: &EchoDB) {
  match msg.method {
    MethodType::Subscribe => {
      subscription_broker::subscribe_broker(uuid, msg, clients).await;
    },
    MethodType::Unsubscribe => {
      subscription_broker::unsubscribe_broker(uuid, msg, clients).await;
    },
    MethodType::Checksum => {
      checksum_broker::checksum_broker(uuid, msg, clients, db).await;
    },
    _ => {
      warn!("{}: unhandled method", uuid);
    },
  }
}