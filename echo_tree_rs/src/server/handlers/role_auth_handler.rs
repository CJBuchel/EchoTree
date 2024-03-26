use protocol::schemas::http_protocol::RoleAuthenticateRequest;

use crate::common::{ClientMap, EchoDB, ResponseResult};


pub async fn role_auth_handler(client_uuid: String, body: RoleAuthenticateRequest, clients: ClientMap, database: EchoDB) -> ResponseResult<impl warp::reply::Reply> {
  // token authentication should already be handled by the auth_token_filter

  // handle new role authentication
  let role_id = body.role_id;
  let password = body.password;

  let db = database.read().await;

  // check authentication
  let role_trees = match db.get_role_manager().authenticate_role(role_id.clone(), password) {
    true => db.get_role_manager().get_role_access(role_id),
    false => {
      return Err(warp::reject::reject());
    },
  };

  // update client role access
  let mut client = match clients.write().await.get(&client_uuid).cloned() {
    Some(client) => client,
    None => {
      return Err(warp::reject::not_found());
    }
  };

  client.role_trees = role_trees;
  clients.write().await.insert(client_uuid, client);

  Ok(warp::reply::reply())
}