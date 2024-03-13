use super::{tree_hierarchy::TreeHierarchy, tree_map::TreeMap};

pub struct DatabaseConfig {
  db_path: String,
  metadata_path: String,
}

impl Default for DatabaseConfig {
  fn default() -> Self {
    DatabaseConfig {
      db_path: "tree.kvdb".to_string(),
      metadata_path: "metadata".to_string(),
    }
  }
}

pub struct Database {
  hierarchy: TreeHierarchy,
  trees: TreeMap,
}

impl Database {
  pub fn new(config: DatabaseConfig) -> Database {
    let db: sled::Db = sled::open(config.db_path.clone()).expect(format!("open failed for {}", config.db_path).as_str());
    let hierarchy = TreeHierarchy::open(&db, config.metadata_path.clone());

    // create the tree map from the hierarchy
    let trees = hierarchy.get_tree_map();

    Database { hierarchy, trees }
  }

  pub fn get_hierarchy(&self) -> &TreeHierarchy {
    &self.hierarchy
  }

  pub fn get_tree_map(&self) -> &TreeMap {
    &self.trees
  }

  // clears all the values in every tree (does not delete the trees themselves)
  pub fn clear(&mut self) {
    self.trees.clear();
    self.hierarchy.clear();
  }

  // drops all the trees, not recoverable unless new hierarchy is created and new trees are opened
  pub fn drop(&mut self) {
    self.trees.drop();
    self.hierarchy.drop();
  }

  pub fn add_tree(&mut self, tree: String, schema: String) {
    self.hierarchy.insert_schema(tree.clone(), schema);
    self.trees.open_tree(tree);
  }

  pub fn remove_tree(&mut self, tree: String) {
    self.hierarchy.remove_schema(tree.clone());
    self.trees.remove_tree(tree);
  }

  pub fn insert(&self, tree: String, key: String, value: String) -> Option<String> {
    match self.trees.get_tree(tree.clone()) {
      Some(tree) => {
        tree.insert(key, value.as_bytes()).expect("insert failed");
        None
      },
      None => Some(format!("tree does not exist: {}", tree))
    }
  }

  pub fn get(&self, tree: String, key: String) -> Option<String> {
    match self.trees.get_tree(tree.clone()) {
      Some(tree) => {
        match tree.get(key) {
          Ok(Some(value)) => Some(std::str::from_utf8(&value).unwrap().to_string()),
          Ok(None) => None,
          Err(e) => Some(format!("get failed: {}", e))
        }
      },
      None => Some(format!("tree does not exist: {}", tree))
    }
  }

  pub fn remove(&self, tree: String, key: String) -> Option<String> {
    match self.trees.get_tree(tree.clone()) {
      Some(tree) => {
        tree.remove(key).expect("remove failed");
        None
      },
      None => Some(format!("tree does not exist: {}", tree))
    }
  }
}