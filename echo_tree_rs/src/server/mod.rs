use crate::common::EchoDB;

mod handlers;
mod filters;

pub async fn server(database: EchoDB, port: u16) {
  let mut addresses: Vec<String> = vec!["localhost".to_string(), "127.0.0.1".to_string(), "0.0.0.0".to_string()];

  let local_ip = local_ip_address::local_ip();
  
  if let Ok(local_ip) = local_ip {
    addresses.push(local_ip.to_string());
  }

  let subject_alt_names = addresses;
  let cert = rcgen::generate_simple_self_signed(subject_alt_names).unwrap();

  // save the files to disk
  std::fs::write("cert.pem", cert.serialize_pem().unwrap()).unwrap();
  std::fs::write("key.rsa", cert.serialize_private_key_pem()).unwrap();

  // create the clients collection
  let clients = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
  let client_routes = filters::client_filter::client_filter(clients.clone(), database.clone(), port);

  let routes = client_routes;

  // encrypted lines
  #[cfg(not(debug_assertions))]
  warp::serve(routes)
    .tls()
    .cert_path("cert.pem")
    .key_path("key.rsa")
    .run(([127, 0, 0, 1], port))
    .await;


  // unencrypted lines
  #[cfg(debug_assertions)]
  warp::serve(routes)
    .run(([0, 0, 0, 0], port))
    .await;
}