use warp::Filter;

use crate::common::{with_clients, with_db, ClientMap, EchoDB};
use crate::server::handlers::register_handlers::{pulse_handler, register_handler, unregister_handler};
use crate::server::handlers::ws_handlers::ws_handler;

// client routes
// 
pub fn client_filter(clients: ClientMap, database: EchoDB, port: u16) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let pulse_route = warp::path("echo_tree").and(warp::path("pulse")).and_then(pulse_handler);

  let register = warp::path("echo_tree").and(warp::path("register"));
  
  let register_routes = register
    .and(warp::post())
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and(with_db(database.clone()))
    .and(warp::any().map(move || port))
    .and_then(register_handler);

  let unregister_routes = register
    .and(warp::delete())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(unregister_handler);

  let ws_route = warp::path("echo_tree").and(warp::path("ws"))
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and(with_db(database.clone()))
    .and_then(ws_handler);

  let cors = warp::cors()
    .allow_any_origin()
    .allow_headers(vec!["content-type"])
    .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

  let routes = pulse_route
    .or(register_routes)
    .or(unregister_routes)
    .or(ws_route)
    .with(cors);

  routes
}