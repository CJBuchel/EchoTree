
// database has `model` as root
// and always has `model/metadata` as the metadata tree

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct Database {
  db: sled::Db,
  hierarchy: sled::Tree, // metadata tree (required for every db instance)
  echo_trees: HashMap<String, sled::Tree>, // all the trees that are not metadata
}

// check if the tree contains a the same parent structure, for example `model/clients` will be true if the tree is `model/clients/1` but not for `test/model/clients`
fn check_tree_contains(tree: String, contains_parents: String) -> bool {
  let tree = tree.split("/").collect::<Vec<&str>>();
  let contains_parents = contains_parents.split("/").collect::<Vec<&str>>();

  for (i, p) in contains_parents.iter().enumerate() {
    if p != &tree[i] {
      return false
    }
  }

  true
}

impl Database {
  pub fn new() -> Database {
    let db: sled::Db = sled::open("tree.kvdb").unwrap();
    let hierarchy = db.open_tree("model/metadata/hierarchy").expect("open_tree failed");
    let mut echo_trees: HashMap<String, sled::Tree> = HashMap::new();

    // check the hierarchy tree and open new trees for each branch
    hierarchy.iter().for_each(|tree| {
      let (k, _) = tree.unwrap_or_default();
      let k = std::str::from_utf8(&k).unwrap();
      println!("opening tree: {}", k);

      if !check_tree_contains(k.to_string(), "model/metadata".to_string()) {
        let t = db.open_tree(k).expect(format!("open_tree failed for {}", k).as_str());
        echo_trees.insert(k.to_string(), t);
      } else {
        println!("skipping metadata tree: {}", k);
      }
    });

    Database { db, hierarchy, echo_trees }
  }

  fn update_hierarchy(&self, tree: String, schema: String) {
    // update the hierarchy tree with the new tree
    // if the tree already exists, update the schema
    let schema = schema.to_string();
    self.hierarchy.insert(tree, schema.as_bytes()).expect("insert failed");
  }

  pub fn get_schema(&self, tree: String) -> String {
    // get the schema from the hierarchy tree
    let schema = self.hierarchy.get(tree).expect("get failed");
    match schema {
      Some(schema) => std::str::from_utf8(&schema).unwrap().to_string(),
      None => "".to_string()
    }
  }

  pub fn add_tree(&mut self, tree: String, schema: String) {
    // add a new tree to the db, and update the hierarchy tree
    if self.echo_trees.contains_key(&tree) {
      println!("tree already exists: {}", tree);
      return
    }

    // check if metadata
    if !check_tree_contains(tree.clone(), "model/metadata".to_string()) {
      let t = self.db.open_tree(tree.clone()).expect(format!("open_tree failed for {}", tree).as_str());
      self.echo_trees.insert(tree.clone(), t);
      self.update_hierarchy(tree, schema);
    } else {
      println!("cannot add metadata tree: {}", tree);
    }
  }

  pub fn remove_tree(&mut self, tree: String) {
    // remove the tree from the db, and update the hierarchy tree
    if !self.echo_trees.contains_key(&tree) {
      println!("tree does not exist: {}", tree);
      return
    }

    self.echo_trees.remove(&tree);
    self.hierarchy.remove(tree).expect("remove failed");
  }

  pub fn insert(&self, tree: String, k: String, v: String) {
    // insert into the desired tree
    let t = self.echo_trees.get(&tree).expect(format!("tree not found: {}", tree).as_str());
    t.insert(k, v.as_bytes()).expect("insert failed");
  }

  pub fn get(&self, tree: String, k: String) -> Option<String> {
    // get the value from the desired tree
    let t = self.echo_trees.get(&tree).expect(format!("tree not found: {}", tree).as_str());
    let v = t.get(k).expect("get failed");
    match v {
      Some(v) => Some(std::str::from_utf8(&v).unwrap().to_string()),
      None => None
    }
  }

  pub fn delete(&self, tree: String, k: String) {
    // delete the value from the desired tree
    let t = self.echo_trees.get(&tree).expect(format!("tree not found: {}", tree).as_str());
    t.remove(k).expect("remove failed");
  }
}