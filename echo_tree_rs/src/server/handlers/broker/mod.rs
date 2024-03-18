use log::warn;
use protocol::schemas::socket_protocol::{OperationRequest, OperationMethodType};

use crate::common::{Clients, EchoDB};

mod subscription_broker;
mod checksum_broker;
mod data_modifier_broker;

/**
 * Broker for the echo message
 * Breakout for each echo message method
 */
pub async fn echo_message_broker(uuid:String, msg: OperationRequest, clients: &Clients, db: &EchoDB) {
  match msg.method {
    OperationMethodType::Subscribe => {
      subscription_broker::subscribe_broker(uuid, msg, clients).await;
    },
    OperationMethodType::Unsubscribe => {
      subscription_broker::unsubscribe_broker(uuid, msg, clients).await;
    },
    OperationMethodType::Checksum => {
      checksum_broker::checksum_broker(uuid, msg, clients, db).await;
    },
    OperationMethodType::Set => {
      data_modifier_broker::set_broker(uuid, msg, clients, db).await;
    },
    OperationMethodType::Get => {
      data_modifier_broker::get_broker(uuid, msg, clients, db).await;
    },
    _ => {
      warn!("{}: unhandled method", uuid);
    },
  }
}