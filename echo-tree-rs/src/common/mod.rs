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
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>, // sender channel to this client
}