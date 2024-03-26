use echo_tree_rs::{common::EchoDB, db, server};
use log::info;
use protocol::schemas::{Role, TestStruct};



#[tokio::main]
async fn main() {
  // for local debugging only
  #[cfg(feature = "logging")]
  {
    // initialize the logger
    // set log level
    pretty_env_logger::init();
  }


  let mut database = db::db::Database::new(db::db::DatabaseConfig::default());

  let server_test = TestStruct {
    test: "lofi beats baby".to_string(),
  };

  let schema = schemars::schema_for!(TestStruct);
  let schema = serde_json::to_string(&schema).unwrap_or_default();

  database.add_tree("test:user".to_string(), schema);


  // serialize the contact and insert it into the database
  let contact_s = serde_json::to_string(&server_test).unwrap_or_default();
  // database.insert("test:user".to_string(), "Server".to_string(), contact_s);

  // create role
  let public_role = Role {
    role_id: "public".to_string(),
    password: "public".to_string(),
    echo_trees: vec!["test:user".to_string()],
  };
  database.get_role_manager().insert_role(public_role.to_owned());

  info!("\n\n -- Data --");
  // print contact
  let contact_from_db = database.get("test:user".to_string(), "Server".to_string()).unwrap_or_default();
  let contact_from_db: TestStruct = serde_json::from_str(&contact_from_db).unwrap_or(TestStruct { test: "".to_string() });
  info!("contact: {:?}", contact_from_db.test);

  // print all schemas
  info!("\n\n -- Schema --");
  database.get_hierarchy().iter().for_each(|result| {
    if let Ok((k, v)) = result {
      // ivec as string
      let k = std::str::from_utf8(&k).unwrap_or_default();
      let v = std::str::from_utf8(&v).unwrap_or_default();
      info!("tree: {}, schema: {}", k, v);
    }
  });

  let database: EchoDB = std::sync::Arc::new(tokio::sync::RwLock::new(database));
  server::server(database, 2121).await;
}