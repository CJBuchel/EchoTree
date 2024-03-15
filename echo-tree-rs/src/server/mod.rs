
mod register_handlers;
mod ws_handlers;
mod client_filter;

pub async fn server() {
  let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
  let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();

  // save the files to disk
  std::fs::write("cert.pem", cert.serialize_pem().unwrap()).unwrap();
  std::fs::write("key.rsa", cert.serialize_private_key_pem()).unwrap();

  // create the clients collection
  let clients = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
  let client_routes = client_filter::client_filter(clients.clone());

  let routes = client_routes;

  warp::serve(routes)
    .tls()
    .cert_path("cert.pem")
    .key_path("key.rsa")
    .run(([127, 0, 0, 1], 2121))
    .await;
}