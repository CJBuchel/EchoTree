use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
struct Contact {
  name: String,
  email: String,
  phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
  username: String,
  password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaData {
  hierarchy: serde_json::Value, // This is a JSON object that represents the full db tree
}

fn main() {
  let db: sled::Db = sled::open("stash.kvdb").unwrap();
  let tree = db.open_tree("model/client").expect("open_tree failed");
}