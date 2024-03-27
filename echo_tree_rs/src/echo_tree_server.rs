

use std::{collections::HashMap, net::Ipv4Addr};

use protocol::schemas::socket_protocol::server_socket_protocol::{EchoItemEvent, EchoTreeEventTree};

use crate::{common::{ClientMap, EchoDB, client_echo::ClientEcho}, db::db::{Database, DatabaseConfig}, server::filters};

pub struct EchoTreeServerConfig {
  db_path: String,
  port: u16,
  addr: Ipv4Addr, // [0,0,0,0] etc...
}

impl Default for EchoTreeServerConfig {
  fn default() -> Self {
    EchoTreeServerConfig {
      db_path: "tree.kvdb".to_string(),
      port: 2121,
      addr: [127,0,0,1].into(),
    }
  }
}


pub struct EchoTreeServer {
  database: EchoDB,
  config: EchoTreeServerConfig,
  clients: ClientMap,
}

impl EchoTreeServer {
  pub fn new(config: EchoTreeServerConfig) -> Self {
    let db_config = DatabaseConfig {
      db_path: config.db_path.clone(),
      metadata_path: "metadata".to_string(),
    };


    let database: EchoDB = std::sync::Arc::new(tokio::sync::RwLock::new(Database::new(db_config)));
    let echo_tree_clients = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
    EchoTreeServer {
      database,
      config,
      clients: echo_tree_clients,
    }
  }

  pub async fn get_role_manager(&self) -> crate::db::role_manager::RoleManager {
    self.database.read().await.get_role_manager().clone()
  }

  pub fn get_clients(&self) -> ClientMap {
    self.clients.clone()
  }

  pub fn get_internal_routes(&self) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::client_filter::client_filter(self.clients.clone(), self.database.clone(), self.config.port)
  }

  pub async fn serve(&self) {
    warp::serve(self.get_internal_routes())
      .run((self.config.addr.clone(), self.config.port)).await
  }

  pub async fn serve_tls(&self, cert_path: &str, key_path: &str) {
    warp::serve(self.get_internal_routes())
      .tls()
      .cert_path(cert_path)
      .key_path(key_path)
      .run((self.config.addr.clone(), self.config.port)).await
  }

  pub async fn set_entry(&self, tree_name: String, key: String, value: String) {
    let mut db = self.database.write().await;
    match db.insert(tree_name.clone(), key.clone(), value.clone()) {
      Some(_) => {
        // echo back the value
        let echo_event = EchoItemEvent {
          tree_name: tree_name.clone(),
          key: key.clone(),
          data: value.clone(),
        };

        self.clients.read().await.echo_item(echo_event);
      },
      None => {
        log::debug!("key not found: {}", key);
      },
    }
  }

  pub async fn get_entry(&self, tree_name: String, key: String) -> Option<String> {
    let db = self.database.read().await;
    db.get(tree_name, key)
  }

  pub async fn remove_entry(&self, tree_name: String, key: String) -> Option<String> {
    let mut db = self.database.write().await;
    match db.remove(tree_name.clone(), key.clone()) {
      Some(v) => {

        // echo back the value
        let echo_event = EchoItemEvent {
          tree_name: tree_name.clone(),
          key: key.clone(),
          data: "".to_string(),
        };

        self.clients.read().await.echo_item(echo_event);
        Some(v)
      },
      None => {
        log::debug!("key not found: {}", key);
        None
      },
    }
  }

  pub async fn set_tree(&self, tree_name: String, tree: HashMap<String, String>) {
    let mut db = self.database.write().await;
    match db.get_tree_map_mut().get_tree_mut(tree_name.clone()) {
      Some(t) => {
        match t.set_from_hashmap(tree) {
          Ok(_) => {
            match t.get_as_hashmap() {
              Ok(v) => {
                let echo_event = EchoTreeEventTree {
                  tree_name: tree_name.clone(),
                  tree: v,
                };
    
                self.clients.read().await.echo_tree(vec![echo_event]);
              },
              Err(e) => {
                log::error!("error getting tree as hashmap: {:?}", e);
              },
            };
          },
          Err(e) => {
            log::error!("error setting tree from hashmap: {:?}", e);
          },
        };
  
      },
      None => {
        log::debug!("tree not found: {}", tree_name);
      },
    };
  }

  pub async fn add_tree(&mut self, tree_name: String, schema: String) {
    let mut db = self.database.write().await;
    db.add_tree(tree_name.clone(), schema);

    let echo_event = EchoTreeEventTree {
      tree_name: tree_name.clone(),
      tree: HashMap::new(),
    };

    self.clients.read().await.echo_tree(vec![echo_event]);
  }

  pub async fn remove_tree(&mut self, tree_name: String) {
    let mut db = self.database.write().await;
    db.remove_tree(tree_name.clone());

    let echo_event = EchoTreeEventTree {
      tree_name: tree_name.clone(),
      tree: HashMap::new(),
    };

    self.clients.read().await.echo_tree(vec![echo_event]);
  }

  pub async fn clear(&mut self) {
    let mut db = self.database.write().await;
    db.clear();
  }

  pub async fn drop(&mut self) {
    let mut db = self.database.write().await;
    db.drop_db();
  }

  pub async fn get_schema(&self, tree_name: String) -> String {
    let db = self.database.read().await;
    db.get_hierarchy().get_schema(tree_name)
  }
}