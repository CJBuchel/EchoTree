use echo_tree_rs::{db, protocol::EchoDB, server};
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};



#[derive(JsonSchema, Serialize, Deserialize, Debug)]
struct Contact {
  name: String,
  email: String,
  phone: String,
}

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

  let contact = Contact {
    name: "John Doe".to_string(),
    email: "".to_string(),
    phone: "123-456-7890".to_string(),
  };

  let schema = schemars::schema_for!(Contact);
  let schema = serde_json::to_string(&schema).unwrap();

  database.add_tree("model/clients".to_string(), schema);


  // serialize the contact and insert it into the database
  let contact_s = serde_json::to_string(&contact).unwrap();
  database.insert("model/clients".to_string(), contact.name, contact_s);

  info!("\n\n -- Data --");
  // print contact
  let contact_from_db = database.get("model/clients".to_string(), "John Doe".to_string()).unwrap();
  let contact_from_db: Contact = serde_json::from_str(&contact_from_db).unwrap();
  info!("contact: {:?}", contact_from_db.name);

  // print all schemas
  info!("\n\n -- Schema --");
  database.get_hierarchy().iter().for_each(|result| {
    if let Ok((k, v)) = result {
      // ivec as string
      let k = std::str::from_utf8(&k).unwrap();
      let v = std::str::from_utf8(&v).unwrap();
      info!("tree: {}, schema: {}", k, v);
    }
  });

  let database: EchoDB = std::sync::Arc::new(tokio::sync::RwLock::new(database));
  server::server(database).await;
}