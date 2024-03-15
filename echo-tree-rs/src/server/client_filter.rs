use warp::Filter;

use crate::protocol::{with_clients, Clients};

use super::{register_handlers::{self, pulse_handler, register_handler}, ws_handlers};



// client routes
// 
pub fn client_filter(clients: Clients) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let pulse_route = warp::path("pulse").and_then(pulse_handler);

  let register = warp::path("register");
  
  let register_routes = register
    .and(warp::post())
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(register_handler);

  let unregister_routes = register
    .and(warp::delete())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(register_handlers::unregister_handler);

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(ws_handlers::ws_handler);

  let routes = pulse_route
    .or(register_routes)
    .or(unregister_routes)
    .or(ws_route)
    .with(warp::cors().allow_any_origin());

  routes
}